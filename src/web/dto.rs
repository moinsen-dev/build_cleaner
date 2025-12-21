use serde::{Deserialize, Serialize};

use crate::cache::{Cache, CacheCategory};
use crate::project::{Artifact, Project, ProjectType};

/// API response wrapper
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message.into()),
        }
    }
}

/// Status response
#[derive(Serialize)]
pub struct StatusDto {
    pub scanning: bool,
    pub has_results: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_scan: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_path: Option<String>,
}

/// Complete scan results
#[derive(Serialize, Clone)]
pub struct ScanResultsDto {
    pub projects: Vec<ProjectDto>,
    pub caches: Vec<CacheDto>,
    pub summary: SummaryDto,
}

/// Project data for JSON serialization
#[derive(Serialize, Clone)]
pub struct ProjectDto {
    pub id: usize,
    pub name: String,
    pub project_type: String,
    pub type_label: String,
    pub type_icon: String,
    pub path: String,
    pub artifacts: Vec<ArtifactDto>,
    pub total_size: u64,
    pub size_display: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<i64>,
}

/// Artifact data
#[derive(Serialize, Clone)]
pub struct ArtifactDto {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub size_display: String,
}

/// Cache data for JSON serialization
#[derive(Serialize, Clone)]
pub struct CacheDto {
    pub id: usize,
    pub cache_type: String,
    pub category: String,
    pub category_label: String,
    pub category_icon: String,
    pub icon: String,
    pub label: String,
    pub description: String,
    pub path: String,
    pub size: u64,
    pub size_display: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cleanup_command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_app_closed: Option<String>,
}

/// Summary statistics
#[derive(Serialize, Clone)]
pub struct SummaryDto {
    pub total_projects: usize,
    pub total_caches: usize,
    pub total_artifacts: usize,
    pub project_size: u64,
    pub cache_size: u64,
    pub total_size: u64,
    pub total_size_display: String,
    pub by_ecosystem: Vec<EcosystemStatsDto>,
    pub by_cache_category: Vec<CategoryStatsDto>,
    pub largest_items: Vec<LargestItemDto>,
}

/// Stats per ecosystem
#[derive(Serialize, Clone)]
pub struct EcosystemStatsDto {
    pub ecosystem: String,
    pub label: String,
    pub icon: String,
    pub count: usize,
    pub size: u64,
    pub size_display: String,
    pub percentage: f64,
}

/// Stats per cache category
#[derive(Serialize, Clone)]
pub struct CategoryStatsDto {
    pub category: String,
    pub label: String,
    pub icon: String,
    pub count: usize,
    pub size: u64,
    pub size_display: String,
    pub percentage: f64,
}

/// Largest items for top-10 display
#[derive(Serialize, Clone)]
pub struct LargestItemDto {
    pub rank: usize,
    pub name: String,
    pub icon: String,
    pub item_type: String, // "project" or "cache"
    pub size: u64,
    pub size_display: String,
    pub path: String,
}

/// Scan request parameters
#[derive(Deserialize, Default)]
pub struct ScanParams {
    #[serde(default = "default_path")]
    pub path: String,
    #[serde(default)]
    pub user_caches: bool,
    #[serde(default)]
    pub cache_only: bool,
    #[serde(default)]
    pub lang: Vec<String>,
    #[serde(default)]
    pub cache_category: Vec<String>,
    #[serde(default)]
    pub min_age: Option<u64>,
    #[serde(default)]
    pub min_size: Option<String>,
}

fn default_path() -> String {
    ".".to_string()
}

/// Cleanup request
#[derive(Deserialize)]
pub struct CleanupRequest {
    #[serde(default)]
    pub project_ids: Vec<usize>,
    #[serde(default)]
    pub cache_ids: Vec<usize>,
}

/// Cleanup response
#[derive(Serialize)]
pub struct CleanupResponse {
    pub deleted_count: usize,
    pub freed_bytes: u64,
    pub freed_display: String,
    pub errors: Vec<String>,
}

/// Script generation request
#[derive(Deserialize)]
pub struct ScriptRequest {
    #[serde(default)]
    pub project_ids: Vec<usize>,
    #[serde(default)]
    pub cache_ids: Vec<usize>,
}

/// Script generation response
#[derive(Serialize)]
pub struct ScriptResponse {
    pub script: String,
}

// Conversion implementations

impl From<&Artifact> for ArtifactDto {
    fn from(a: &Artifact) -> Self {
        Self {
            name: a.name.clone(),
            path: a.path.display().to_string(),
            size: a.size,
            size_display: a.size_display(),
        }
    }
}

impl ProjectDto {
    pub fn from_project(p: &Project, id: usize) -> Self {
        Self {
            id,
            name: p.name.clone(),
            project_type: format!("{:?}", p.project_type),
            type_label: p.project_type.label().to_string(),
            type_icon: p.project_type.icon().to_string(),
            path: p.path.display().to_string(),
            artifacts: p.artifacts.iter().map(ArtifactDto::from).collect(),
            total_size: p.total_size,
            size_display: p.size_display(),
            last_modified: p.last_modified,
        }
    }
}

impl CacheDto {
    pub fn from_cache(c: &Cache, id: usize) -> Self {
        Self {
            id,
            cache_type: format!("{:?}", c.cache_type),
            category: format!("{:?}", c.cache_type.category()),
            category_label: c.cache_type.category().label().to_string(),
            category_icon: c.cache_type.category().icon().to_string(),
            icon: c.cache_type.icon().to_string(),
            label: c.cache_type.label().to_string(),
            description: c.cache_type.description().to_string(),
            path: c.path.display().to_string(),
            size: c.size,
            size_display: c.size_display(),
            cleanup_command: c.cache_type.cleanup_command().map(|(cmd, _)| cmd.to_string()),
            requires_app_closed: c.cache_type.requires_app_closed().map(|s| s.to_string()),
        }
    }
}

impl SummaryDto {
    pub fn from_results(projects: &[Project], caches: &[Cache]) -> Self {
        use std::collections::HashMap;

        let project_size: u64 = projects.iter().map(|p| p.total_size).sum();
        let cache_size: u64 = caches.iter().map(|c| c.size).sum();
        let total_size = project_size + cache_size;
        let total_artifacts: usize = projects.iter().map(|p| p.artifacts.len()).sum();

        // Group by ecosystem
        let mut ecosystem_stats: HashMap<ProjectType, (usize, u64)> = HashMap::new();
        for p in projects {
            let entry = ecosystem_stats.entry(p.project_type).or_insert((0, 0));
            entry.0 += 1;
            entry.1 += p.total_size;
        }

        let mut by_ecosystem: Vec<EcosystemStatsDto> = ecosystem_stats
            .into_iter()
            .map(|(pt, (count, size))| EcosystemStatsDto {
                ecosystem: format!("{:?}", pt),
                label: pt.label().to_string(),
                icon: pt.icon().to_string(),
                count,
                size,
                size_display: bytesize::ByteSize(size).to_string(),
                percentage: if total_size > 0 {
                    (size as f64 / total_size as f64) * 100.0
                } else {
                    0.0
                },
            })
            .collect();
        by_ecosystem.sort_by(|a, b| b.size.cmp(&a.size));

        // Group by cache category
        let mut category_stats: HashMap<CacheCategory, (usize, u64)> = HashMap::new();
        for c in caches {
            let entry = category_stats
                .entry(c.cache_type.category())
                .or_insert((0, 0));
            entry.0 += 1;
            entry.1 += c.size;
        }

        let mut by_cache_category: Vec<CategoryStatsDto> = category_stats
            .into_iter()
            .map(|(cat, (count, size))| CategoryStatsDto {
                category: format!("{:?}", cat),
                label: cat.label().to_string(),
                icon: cat.icon().to_string(),
                count,
                size,
                size_display: bytesize::ByteSize(size).to_string(),
                percentage: if total_size > 0 {
                    (size as f64 / total_size as f64) * 100.0
                } else {
                    0.0
                },
            })
            .collect();
        by_cache_category.sort_by(|a, b| b.size.cmp(&a.size));

        // Top 10 largest items
        let mut all_items: Vec<LargestItemDto> = Vec::new();

        for p in projects.iter() {
            all_items.push(LargestItemDto {
                rank: 0,
                name: p.name.clone(),
                icon: p.project_type.icon().to_string(),
                item_type: "project".to_string(),
                size: p.total_size,
                size_display: p.size_display(),
                path: p.path.display().to_string(),
            });
        }

        for c in caches.iter() {
            all_items.push(LargestItemDto {
                rank: 0,
                name: c.cache_type.label().to_string(),
                icon: c.cache_type.icon().to_string(),
                item_type: "cache".to_string(),
                size: c.size,
                size_display: c.size_display(),
                path: c.path.display().to_string(),
            });
        }

        all_items.sort_by(|a, b| b.size.cmp(&a.size));
        let largest_items: Vec<LargestItemDto> = all_items
            .into_iter()
            .take(10)
            .enumerate()
            .map(|(i, mut item)| {
                item.rank = i + 1;
                item
            })
            .collect();

        Self {
            total_projects: projects.len(),
            total_caches: caches.len(),
            total_artifacts,
            project_size,
            cache_size,
            total_size,
            total_size_display: bytesize::ByteSize(total_size).to_string(),
            by_ecosystem,
            by_cache_category,
            largest_items,
        }
    }
}

impl ScanResultsDto {
    pub fn from_results(projects: &[Project], caches: &[Cache]) -> Self {
        Self {
            projects: projects
                .iter()
                .enumerate()
                .map(|(i, p)| ProjectDto::from_project(p, i))
                .collect(),
            caches: caches
                .iter()
                .enumerate()
                .map(|(i, c)| CacheDto::from_cache(c, i))
                .collect(),
            summary: SummaryDto::from_results(projects, caches),
        }
    }
}
