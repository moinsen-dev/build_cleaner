use crate::project::{Artifact, Config, Project, ProjectType};
use anyhow::Result;
use rayon::prelude::*;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::SystemTime;
use walkdir::{DirEntry, WalkDir};

/// Directories to always skip during scanning
const SKIP_DIRS: &[&str] = &[
    ".git",
    ".hg",
    ".svn",
    ".DS_Store",
    "$RECYCLE.BIN",
    "System Volume Information",
];

/// System directories to never scan into
const SYSTEM_DIRS: &[&str] = &[
    "/System",
    "/Library",
    "/Applications",
    "/usr",
    "/bin",
    "/sbin",
    "/var",
    "/private",
];

/// Scanner for finding projects and their build artifacts
pub struct Scanner {
    config: Config,
}

impl Scanner {
    /// Create a new scanner with the given configuration
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Scan the root directory for projects with build artifacts
    pub fn scan(&self) -> Result<Vec<Project>> {
        let root = self.config.root_dir.canonicalize()?;

        // Check if scanning a system directory
        let root_str = root.to_string_lossy();
        for sys_dir in SYSTEM_DIRS {
            if root_str.starts_with(sys_dir) {
                anyhow::bail!(
                    "Refusing to scan system directory: {}. \
                    Please specify a user projects directory.",
                    root.display()
                );
            }
        }

        let projects = Mutex::new(Vec::new());

        // Walk the directory tree
        let entries: Vec<_> = WalkDir::new(&root)
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| self.should_enter(e))
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_dir())
            .collect();

        // Process entries in parallel
        entries.par_iter().for_each(|entry| {
            if let Some(project) = self.detect_project(entry.path()) {
                let mut projects = projects.lock().unwrap();
                projects.push(project);
            }
        });

        let mut result = projects.into_inner().unwrap();

        // Apply filters
        result = self.apply_filters(result);

        // Sort by size (largest first)
        result.sort_by(|a, b| b.total_size.cmp(&a.total_size));

        Ok(result)
    }

    /// Check if we should enter this directory during traversal
    fn should_enter(&self, entry: &DirEntry) -> bool {
        let name = entry.file_name().to_string_lossy();

        // Skip hidden directories (except some we care about like .dart_tool)
        if name.starts_with('.') && ![".", ".dart_tool", ".gradle", ".venv"].contains(&name.as_ref())
        {
            // Allow .dart_tool etc. to be detected as artifacts later
            if SKIP_DIRS.contains(&name.as_ref()) {
                return false;
            }
        }

        // Skip VCS directories
        if SKIP_DIRS.contains(&name.as_ref()) {
            return false;
        }

        // Skip user-excluded patterns
        for pattern in &self.config.exclude_patterns {
            if name.contains(pattern.as_str()) {
                return false;
            }
        }

        // Skip known artifact directories to avoid deep scanning
        // We'll detect these at the project level instead
        let artifact_dirs = [
            "node_modules",
            "target",
            "__pycache__",
            "build",
            "dist",
            "bin",
            "obj",
            "vendor",
            ".dart_tool",
            ".gradle",
            "Pods",
        ];

        // Don't descend into artifact dirs - we'll handle them at project detection
        !artifact_dirs.contains(&name.as_ref())
    }

    /// Try to detect a project at the given path
    fn detect_project(&self, path: &Path) -> Option<Project> {
        for project_type in ProjectType::all() {
            if self.is_project_type(path, *project_type) {
                if let Some(project) = self.build_project(path, *project_type) {
                    // Only include if has artifacts
                    if !project.artifacts.is_empty() {
                        return Some(project);
                    }
                }
            }
        }
        None
    }

    /// Check if a directory is a project of the given type
    fn is_project_type(&self, path: &Path, project_type: ProjectType) -> bool {
        match project_type {
            ProjectType::DotNet => {
                // Special handling for .NET: check for *.csproj or *.sln
                if let Ok(entries) = fs::read_dir(path) {
                    for entry in entries.flatten() {
                        let name = entry.file_name();
                        let name = name.to_string_lossy();
                        if name.ends_with(".csproj")
                            || name.ends_with(".sln")
                            || name.ends_with(".fsproj")
                            || name.ends_with(".vbproj")
                        {
                            return true;
                        }
                    }
                }
                false
            }
            _ => {
                // Check for config file patterns
                for pattern in project_type.config_patterns() {
                    if path.join(pattern).exists() {
                        return true;
                    }
                }
                false
            }
        }
    }

    /// Build a Project struct for a detected project
    fn build_project(&self, path: &Path, project_type: ProjectType) -> Option<Project> {
        let name = path
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string());

        let mut project = Project::new(name, project_type, path.to_path_buf());

        // Find artifact directories
        for pattern in project_type.artifact_patterns() {
            // Handle nested patterns like "ios/Pods"
            let artifact_path = if pattern.contains('/') {
                path.join(pattern)
            } else if pattern.contains('*') {
                // Handle glob patterns like "*.egg-info"
                if let Some(artifacts) = self.find_glob_artifacts(path, pattern) {
                    for artifact in artifacts {
                        project.add_artifact(artifact);
                    }
                }
                continue;
            } else {
                path.join(pattern)
            };

            if artifact_path.exists() && artifact_path.is_dir() {
                let size = self.calculate_dir_size(&artifact_path);
                let artifact = Artifact::new(artifact_path, size);
                project.add_artifact(artifact);
            }
        }

        // Also handle __pycache__ recursively for Python
        if project_type == ProjectType::Python {
            self.find_pycache_dirs(path, &mut project);
        }

        // Get last modification time
        if let Ok(metadata) = fs::metadata(path) {
            if let Ok(modified) = metadata.modified() {
                if let Ok(duration) = modified.duration_since(SystemTime::UNIX_EPOCH) {
                    project.last_modified = Some(duration.as_secs() as i64);
                }
            }
        }

        Some(project)
    }

    /// Find artifacts matching a glob pattern (e.g., "*.egg-info")
    fn find_glob_artifacts(&self, path: &Path, pattern: &str) -> Option<Vec<Artifact>> {
        if !pattern.starts_with('*') {
            return None;
        }

        let suffix = &pattern[1..]; // e.g., ".egg-info"
        let mut artifacts = Vec::new();

        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let name = entry.file_name();
                let name = name.to_string_lossy();
                if name.ends_with(suffix) {
                    let artifact_path = entry.path();
                    if artifact_path.is_dir() {
                        let size = self.calculate_dir_size(&artifact_path);
                        artifacts.push(Artifact::new(artifact_path, size));
                    }
                }
            }
        }

        if artifacts.is_empty() {
            None
        } else {
            Some(artifacts)
        }
    }

    /// Recursively find __pycache__ directories
    fn find_pycache_dirs(&self, path: &Path, project: &mut Project) {
        let visited: HashSet<PathBuf> = project.artifacts.iter().map(|a| a.path.clone()).collect();

        for entry in WalkDir::new(path)
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| {
                let name = e.file_name().to_string_lossy();
                // Don't descend into venv or other artifact dirs
                !["venv", ".venv", "node_modules", ".git"].contains(&name.as_ref())
            })
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_dir() {
                let name = entry.file_name().to_string_lossy();
                if name == "__pycache__" || name == ".pytest_cache" {
                    let artifact_path = entry.path().to_path_buf();
                    if !visited.contains(&artifact_path) {
                        let size = self.calculate_dir_size(&artifact_path);
                        project.add_artifact(Artifact::new(artifact_path, size));
                    }
                }
            }
        }
    }

    /// Calculate the total size of a directory
    fn calculate_dir_size(&self, path: &Path) -> u64 {
        WalkDir::new(path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter_map(|e| e.metadata().ok())
            .map(|m| m.len())
            .sum()
    }

    /// Apply configured filters to the project list
    fn apply_filters(&self, projects: Vec<Project>) -> Vec<Project> {
        projects
            .into_iter()
            .filter(|p| self.passes_filters(p))
            .collect()
    }

    /// Check if a project passes all configured filters
    fn passes_filters(&self, project: &Project) -> bool {
        // Filter by project type
        if !self.config.project_filters.is_empty()
            && !self.config.project_filters.contains(&project.project_type)
        {
            return false;
        }

        // Filter by minimum size
        if let Some(min_size) = self.config.min_size_bytes {
            if project.total_size < min_size {
                return false;
            }
        }

        // Filter by minimum age
        if let Some(min_age_days) = self.config.min_age_days {
            if let Some(last_modified) = project.last_modified {
                let now = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64;
                let age_secs = now - last_modified;
                let age_days = age_secs / (24 * 60 * 60);
                if age_days < min_age_days as i64 {
                    return false;
                }
            }
        }

        true
    }
}
