use axum::{
    extract::{Query, State},
    response::sse::{Event, Sse},
    Json,
};
use futures::stream::Stream;
use std::convert::Infallible;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use crate::cache::{Cache, CacheCategory};
use crate::cache_scanner::CacheScanner;
use crate::project::{Config, Project, ProjectType};
use crate::scanner::Scanner;
use crate::script::ScriptGenerator;

use super::dto::*;
use super::state::{AppState, ScanProgress, ScanResults};

/// GET /api/status - Get current scanning state
pub async fn get_status(State(state): State<Arc<AppState>>) -> Json<ApiResponse<StatusDto>> {
    let is_scanning = *state.is_scanning.read().await;
    let results = state.scan_results.read().await;

    let status = StatusDto {
        scanning: is_scanning,
        has_results: results.is_some(),
        last_scan: results.as_ref().map(|r| r.scan_time),
        scan_path: results.as_ref().map(|r| r.scan_path.display().to_string()),
    };

    Json(ApiResponse::success(status))
}

/// GET /api/scan - Trigger a new scan
pub async fn trigger_scan(
    State(state): State<Arc<AppState>>,
    Query(params): Query<ScanParams>,
) -> Json<ApiResponse<StatusDto>> {
    // Check if already scanning
    {
        let is_scanning = state.is_scanning.read().await;
        if *is_scanning {
            return Json(ApiResponse::error("Scan already in progress"));
        }
    }

    // Mark as scanning
    {
        let mut is_scanning = state.is_scanning.write().await;
        *is_scanning = true;
    }

    // Clone values needed for response before moving params
    let scan_path_response = params.path.clone();

    // Clone state for async task
    let state_clone = state.clone();

    // Spawn scan task
    tokio::spawn(async move {
        // Send initial progress
        state_clone.send_progress(ScanProgress::scanning(&params.path));

        let result = perform_scan_with_progress(&params, &state_clone).await;

        match result {
            Ok((projects, caches, path)) => {
                let total_size: u64 = projects.iter().map(|p| p.total_size).sum::<u64>()
                    + caches.iter().map(|c| c.size).sum::<u64>();

                // Send completion progress
                state_clone.send_progress(ScanProgress::complete(
                    projects.len(),
                    caches.len(),
                    total_size,
                ));

                let mut scan_results = state_clone.scan_results.write().await;
                *scan_results = Some(ScanResults::new(projects, caches, path));
            }
            Err(e) => {
                eprintln!("Scan error: {}", e);
                state_clone.send_progress(ScanProgress {
                    current_path: String::new(),
                    projects_found: 0,
                    caches_found: 0,
                    total_size: 0,
                    complete: true,
                    message: Some(format!("Scan error: {}", e)),
                });
            }
        }

        let mut is_scanning = state_clone.is_scanning.write().await;
        *is_scanning = false;
    });

    Json(ApiResponse::success(StatusDto {
        scanning: true,
        has_results: false,
        last_scan: None,
        scan_path: Some(scan_path_response),
    }))
}

/// GET /api/scan/progress - Server-Sent Events for scan progress
pub async fn scan_progress(
    State(state): State<Arc<AppState>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let mut rx = state.subscribe_progress();

    let stream = async_stream::stream! {
        loop {
            match tokio::time::timeout(Duration::from_secs(30), rx.recv()).await {
                Ok(Ok(progress)) => {
                    let data = serde_json::json!({
                        "current_path": progress.current_path,
                        "projects_found": progress.projects_found,
                        "caches_found": progress.caches_found,
                        "total_size": progress.total_size,
                        "total_size_display": bytesize::ByteSize(progress.total_size).to_string(),
                        "complete": progress.complete,
                        "message": progress.message,
                    });
                    yield Ok(Event::default().data(data.to_string()));

                    if progress.complete {
                        break;
                    }
                }
                Ok(Err(_)) => {
                    // Channel closed
                    break;
                }
                Err(_) => {
                    // Timeout - send keepalive
                    yield Ok(Event::default().comment("keepalive"));
                }
            }
        }
    };

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("keepalive"),
    )
}

/// GET /api/results - Get scan results
pub async fn get_results(
    State(state): State<Arc<AppState>>,
) -> Json<ApiResponse<ScanResultsDto>> {
    let results = state.scan_results.read().await;

    match &*results {
        Some(r) => {
            let dto = ScanResultsDto::from_results(&r.projects, &r.caches);
            Json(ApiResponse::success(dto))
        }
        None => Json(ApiResponse::error("No scan results available. Run a scan first.")),
    }
}

/// POST /api/clean - Execute cleanup for selected items
pub async fn execute_clean(
    State(state): State<Arc<AppState>>,
    Json(request): Json<CleanupRequest>,
) -> Json<ApiResponse<CleanupResponse>> {
    let results = state.scan_results.read().await;

    let results = match &*results {
        Some(r) => r.clone(),
        None => {
            return Json(ApiResponse::error("No scan results available"));
        }
    };

    drop(results); // Release read lock

    // Get the results again for actual cleanup
    let scan_results = state.scan_results.read().await;
    let scan_results = match &*scan_results {
        Some(r) => r.clone(),
        None => return Json(ApiResponse::error("No scan results available")),
    };

    // Collect items to clean
    let projects_to_clean: Vec<&Project> = request
        .project_ids
        .iter()
        .filter_map(|&id| scan_results.projects.get(id))
        .collect();

    let caches_to_clean: Vec<&Cache> = request
        .cache_ids
        .iter()
        .filter_map(|&id| scan_results.caches.get(id))
        .collect();

    if projects_to_clean.is_empty() && caches_to_clean.is_empty() {
        return Json(ApiResponse::error("No valid items selected for cleanup"));
    }

    // Perform cleanup
    let mut deleted_count = 0usize;
    let mut freed_bytes = 0u64;
    let mut errors = Vec::new();

    // Delete project artifacts
    for project in projects_to_clean {
        for artifact in &project.artifacts {
            match std::fs::remove_dir_all(&artifact.path) {
                Ok(_) => {
                    deleted_count += 1;
                    freed_bytes += artifact.size;
                }
                Err(e) => {
                    errors.push(format!(
                        "Failed to delete {}: {}",
                        artifact.path.display(),
                        e
                    ));
                }
            }
        }
    }

    // Delete caches
    for cache in caches_to_clean {
        match std::fs::remove_dir_all(&cache.path) {
            Ok(_) => {
                deleted_count += 1;
                freed_bytes += cache.size;
            }
            Err(e) => {
                errors.push(format!(
                    "Failed to delete {}: {}",
                    cache.path.display(),
                    e
                ));
            }
        }
    }

    Json(ApiResponse::success(CleanupResponse {
        deleted_count,
        freed_bytes,
        freed_display: bytesize::ByteSize(freed_bytes).to_string(),
        errors,
    }))
}

/// POST /api/generate-script - Generate cleanup script
pub async fn generate_script(
    State(state): State<Arc<AppState>>,
    Json(request): Json<ScriptRequest>,
) -> Json<ApiResponse<ScriptResponse>> {
    let results = state.scan_results.read().await;

    let results = match &*results {
        Some(r) => r.clone(),
        None => {
            return Json(ApiResponse::error("No scan results available"));
        }
    };

    // Collect items for script
    let projects_to_clean: Vec<&Project> = request
        .project_ids
        .iter()
        .filter_map(|&id| results.projects.get(id))
        .collect();

    let caches_to_clean: Vec<&Cache> = request
        .cache_ids
        .iter()
        .filter_map(|&id| results.caches.get(id))
        .collect();

    if projects_to_clean.is_empty() && caches_to_clean.is_empty() {
        return Json(ApiResponse::error("No valid items selected for script generation"));
    }

    // Generate script
    let mut generator = ScriptGenerator::new();

    for project in projects_to_clean {
        generator.add_projects(std::slice::from_ref(project));
    }

    for cache in caches_to_clean {
        generator.add_caches(std::slice::from_ref(cache));
    }

    let script = generator.generate();

    Json(ApiResponse::success(ScriptResponse { script }))
}

/// Perform the actual scan with progress updates
async fn perform_scan_with_progress(
    params: &ScanParams,
    state: &Arc<AppState>,
) -> anyhow::Result<(Vec<Project>, Vec<Cache>, PathBuf)> {
    let path = PathBuf::from(&params.path);
    let root_dir = path.canonicalize().unwrap_or(path.clone());
    let root_display = root_dir.display().to_string();

    // Build project filters
    let project_filters: Vec<ProjectType> = params
        .lang
        .iter()
        .filter_map(|s| match s.to_lowercase().as_str() {
            "node" => Some(ProjectType::Node),
            "rust" => Some(ProjectType::Rust),
            "python" => Some(ProjectType::Python),
            "flutter" => Some(ProjectType::Flutter),
            "java-maven" | "javamaven" => Some(ProjectType::JavaMaven),
            "java-gradle" | "javagradle" => Some(ProjectType::JavaGradle),
            "cpp" => Some(ProjectType::Cpp),
            "dot-net" | "dotnet" => Some(ProjectType::DotNet),
            "go" => Some(ProjectType::Go),
            _ => None,
        })
        .collect();

    // Parse min_size
    let min_size_bytes = params.min_size.as_ref().and_then(|s| parse_size(s));

    // Send initial scanning message
    state.send_progress(ScanProgress {
        current_path: root_display.clone(),
        projects_found: 0,
        caches_found: 0,
        total_size: 0,
        complete: false,
        message: Some(format!("Starting scan of {}", root_display)),
    });

    // Scan projects
    let projects = if params.cache_only {
        Vec::new()
    } else {
        let config = Config {
            root_dir: root_dir.clone(),
            dry_run: true,
            auto_confirm: false,
            interactive: false,
            include_patterns: Vec::new(),
            exclude_patterns: Vec::new(),
            project_filters,
            min_age_days: params.min_age,
            min_size_bytes,
            verbose: false,
        };

        // Send progress that we're scanning projects
        state.send_progress(ScanProgress {
            current_path: root_display.clone(),
            projects_found: 0,
            caches_found: 0,
            total_size: 0,
            complete: false,
            message: Some("Scanning for project build artifacts...".to_string()),
        });

        let scanner = Scanner::new(config);
        let state_clone = state.clone();

        // Run scanner in blocking task
        let projects = tokio::task::spawn_blocking(move || scanner.scan())
            .await
            .map_err(|e| anyhow::anyhow!("Task join error: {}", e))??;

        // Send progress for each project found
        let mut total_size = 0u64;
        for (i, project) in projects.iter().enumerate() {
            total_size += project.total_size;
            state_clone.send_progress(ScanProgress::found_project(
                &project.path.display().to_string(),
                &project.name,
                project.project_type.label(),
                project.total_size,
                i + 1,
                total_size,
            ));
            // Small delay to not overwhelm the UI
            tokio::time::sleep(Duration::from_millis(10)).await;
        }

        projects
    };

    let project_size: u64 = projects.iter().map(|p| p.total_size).sum();

    // Scan caches
    let caches = if params.user_caches || params.cache_only {
        state.send_progress(ScanProgress {
            current_path: String::new(),
            projects_found: projects.len(),
            caches_found: 0,
            total_size: project_size,
            complete: false,
            message: Some("Scanning user caches...".to_string()),
        });

        let scanner = match CacheScanner::new() {
            Some(s) => s,
            None => return Ok((projects, Vec::new(), root_dir)),
        };

        // Build cache category filters
        let categories: Vec<CacheCategory> = params
            .cache_category
            .iter()
            .filter_map(|s| match s.to_lowercase().as_str() {
                "ai-ml" | "aiml" => Some(CacheCategory::AiMl),
                "package" => Some(CacheCategory::PackageManager),
                "dev" => Some(CacheCategory::Development),
                "ide" => Some(CacheCategory::Ide),
                "container" => Some(CacheCategory::Container),
                _ => None,
            })
            .collect();

        let scanner = scanner.with_categories(categories).with_min_size(min_size_bytes);

        let state_clone = state.clone();
        let projects_len = projects.len();

        // Run cache scanner in blocking task
        let caches = tokio::task::spawn_blocking(move || scanner.scan())
            .await
            .map_err(|e| anyhow::anyhow!("Task join error: {}", e))?;

        // Send progress for each cache found
        let mut total_cache_size = 0u64;
        for (i, cache) in caches.iter().enumerate() {
            total_cache_size += cache.size;
            state_clone.send_progress(ScanProgress {
                current_path: String::new(),
                projects_found: projects_len,
                caches_found: i + 1,
                total_size: project_size + total_cache_size,
                complete: false,
                message: Some(format!("Found cache: {} ({})", cache.cache_type.label(), bytesize::ByteSize(cache.size))),
            });
            // Small delay to not overwhelm the UI
            tokio::time::sleep(Duration::from_millis(10)).await;
        }

        caches
    } else {
        Vec::new()
    };

    Ok((projects, caches, root_dir))
}

/// Parse size string (copied from cli.rs)
fn parse_size(s: &str) -> Option<u64> {
    let s = s.trim().to_uppercase();

    let (num_str, unit) = s
        .char_indices()
        .find(|(_, c)| c.is_alphabetic())
        .map(|(i, _)| s.split_at(i))
        .unwrap_or((&s, "B"));

    let num: f64 = num_str.trim().parse().ok()?;

    let multiplier: u64 = match unit.trim() {
        "B" | "" => 1,
        "KB" | "K" => 1_000,
        "KIB" => 1_024,
        "MB" | "M" => 1_000_000,
        "MIB" => 1_048_576,
        "GB" | "G" => 1_000_000_000,
        "GIB" => 1_073_741_824,
        "TB" | "T" => 1_000_000_000_000,
        "TIB" => 1_099_511_627_776,
        _ => return None,
    };

    Some((num * multiplier as f64) as u64)
}
