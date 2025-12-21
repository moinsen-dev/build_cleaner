use std::path::PathBuf;
use tokio::sync::{broadcast, RwLock};

use crate::cache::Cache;
use crate::cli::Args;
use crate::project::Project;

/// Scan progress event
#[derive(Clone, Debug)]
pub struct ScanProgress {
    /// Current directory being scanned
    pub current_path: String,
    /// Number of projects found so far
    pub projects_found: usize,
    /// Number of caches found so far
    pub caches_found: usize,
    /// Total size found so far
    pub total_size: u64,
    /// Whether scanning is complete
    pub complete: bool,
    /// Optional message (e.g., "Found Node.js project: my-app")
    pub message: Option<String>,
}

impl ScanProgress {
    pub fn scanning(path: &str) -> Self {
        Self {
            current_path: path.to_string(),
            projects_found: 0,
            caches_found: 0,
            total_size: 0,
            complete: false,
            message: None,
        }
    }

    pub fn found_project(path: &str, project_name: &str, project_type: &str, size: u64, count: usize, total: u64) -> Self {
        Self {
            current_path: path.to_string(),
            projects_found: count,
            caches_found: 0,
            total_size: total,
            complete: false,
            message: Some(format!("Found {} project: {} ({})", project_type, project_name, bytesize::ByteSize(size))),
        }
    }

    pub fn complete(projects: usize, caches: usize, total: u64) -> Self {
        Self {
            current_path: String::new(),
            projects_found: projects,
            caches_found: caches,
            total_size: total,
            complete: true,
            message: Some("Scan complete!".to_string()),
        }
    }
}

/// Shared application state
#[allow(dead_code)]
pub struct AppState {
    /// Current scan results
    pub scan_results: RwLock<Option<ScanResults>>,
    /// Whether a scan is currently in progress
    pub is_scanning: RwLock<bool>,
    /// Initial CLI arguments (for defaults)
    pub initial_args: Args,
    /// Broadcast channel for scan progress
    pub progress_sender: broadcast::Sender<ScanProgress>,
}

impl AppState {
    pub fn new(args: Args) -> Self {
        let (progress_sender, _) = broadcast::channel(100);
        Self {
            scan_results: RwLock::new(None),
            is_scanning: RwLock::new(false),
            initial_args: args,
            progress_sender,
        }
    }

    pub fn subscribe_progress(&self) -> broadcast::Receiver<ScanProgress> {
        self.progress_sender.subscribe()
    }

    pub fn send_progress(&self, progress: ScanProgress) {
        // Ignore send errors (no subscribers)
        let _ = self.progress_sender.send(progress);
    }
}

/// Results from a scan operation
#[derive(Clone)]
pub struct ScanResults {
    pub projects: Vec<Project>,
    pub caches: Vec<Cache>,
    pub scan_path: PathBuf,
    pub scan_time: i64,
}

impl ScanResults {
    pub fn new(projects: Vec<Project>, caches: Vec<Cache>, scan_path: PathBuf) -> Self {
        Self {
            projects,
            caches,
            scan_path,
            scan_time: chrono::Utc::now().timestamp(),
        }
    }
}
