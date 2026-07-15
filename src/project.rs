use std::fmt;
use std::path::PathBuf;
use std::time::SystemTime;

/// Supported project types/ecosystems
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProjectType {
    /// Node.js/JavaScript projects (package.json)
    Node,
    /// Rust projects (Cargo.toml)
    Rust,
    /// Python projects (pyproject.toml, setup.py, requirements.txt)
    Python,
    /// Flutter/Dart projects (pubspec.yaml)
    Flutter,
    /// Java Maven projects (pom.xml)
    JavaMaven,
    /// Java Gradle projects (build.gradle)
    JavaGradle,
    /// C/C++ projects (CMakeLists.txt, Makefile)
    Cpp,
    /// .NET/C# projects (*.csproj, *.sln)
    DotNet,
    /// Go projects (go.mod)
    Go,
}

impl ProjectType {
    /// Get the emoji/icon for this project type
    pub fn icon(&self) -> &'static str {
        match self {
            ProjectType::Node => "\u{1F4E6}",     // Package
            ProjectType::Rust => "\u{1F980}",     // Crab
            ProjectType::Python => "\u{1F40D}",   // Snake
            ProjectType::Flutter => "\u{1F98B}",  // Butterfly (closest to Flutter bird)
            ProjectType::JavaMaven => "\u{2615}", // Coffee
            ProjectType::JavaGradle => "\u{2615}", // Coffee
            ProjectType::Cpp => "\u{2699}",       // Gear
            ProjectType::DotNet => "\u{1F7E3}",   // Purple circle
            ProjectType::Go => "\u{1F439}",       // Hamster (Go gopher-ish)
        }
    }

    /// Get the display label for this project type
    pub fn label(&self) -> &'static str {
        match self {
            ProjectType::Node => "Node.js",
            ProjectType::Rust => "Rust",
            ProjectType::Python => "Python",
            ProjectType::Flutter => "Flutter",
            ProjectType::JavaMaven => "Maven",
            ProjectType::JavaGradle => "Gradle",
            ProjectType::Cpp => "C/C++",
            ProjectType::DotNet => ".NET",
            ProjectType::Go => "Go",
        }
    }

    /// Get the config file patterns that indicate this project type
    pub fn config_patterns(&self) -> &'static [&'static str] {
        match self {
            ProjectType::Node => &["package.json"],
            ProjectType::Rust => &["Cargo.toml"],
            ProjectType::Python => &["pyproject.toml", "setup.py", "setup.cfg", "requirements.txt"],
            ProjectType::Flutter => &["pubspec.yaml"],
            ProjectType::JavaMaven => &["pom.xml"],
            ProjectType::JavaGradle => &["build.gradle", "build.gradle.kts"],
            ProjectType::Cpp => &["CMakeLists.txt", "Makefile", "makefile"],
            ProjectType::DotNet => &[], // Special handling: *.csproj, *.sln
            ProjectType::Go => &["go.mod"],
        }
    }

    /// Get the artifact directory names to clean for this project type
    pub fn artifact_patterns(&self) -> &'static [&'static str] {
        match self {
            ProjectType::Node => &["node_modules", "dist", "build", ".next", ".nuxt"],
            ProjectType::Rust => &["target"],
            ProjectType::Python => &[
                "__pycache__",
                ".pytest_cache",
                ".tox",
                "build",
                "dist",
                ".eggs",
                "*.egg-info",
                "venv",
                ".venv",
            ],
            ProjectType::Flutter => &["build", ".dart_tool", "ios/Pods"],
            ProjectType::JavaMaven => &["target"],
            ProjectType::JavaGradle => &["build", ".gradle"],
            ProjectType::Cpp => &["build", "cmake-build-debug", "cmake-build-release"],
            ProjectType::DotNet => &["bin", "obj"],
            ProjectType::Go => &["vendor"],
        }
    }

    /// Get all project types
    pub fn all() -> &'static [ProjectType] {
        &[
            ProjectType::Node,
            ProjectType::Rust,
            ProjectType::Python,
            ProjectType::Flutter,
            ProjectType::JavaMaven,
            ProjectType::JavaGradle,
            ProjectType::Cpp,
            ProjectType::DotNet,
            ProjectType::Go,
        ]
    }
}

impl fmt::Display for ProjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.icon(), self.label())
    }
}

/// Represents a discovered project with its build artifacts
#[derive(Debug, Clone)]
pub struct Project {
    /// Project name (derived from directory name or config file)
    pub name: String,
    /// Type of project
    pub project_type: ProjectType,
    /// Path to the project root directory
    pub path: PathBuf,
    /// Artifact directories/files to be deleted
    pub artifacts: Vec<Artifact>,
    /// Total size of all artifacts in bytes
    pub total_size: u64,
    /// Last modification time (Unix timestamp)
    pub last_modified: Option<i64>,
}

impl Project {
    /// Create a new project
    pub fn new(name: String, project_type: ProjectType, path: PathBuf) -> Self {
        Self {
            name,
            project_type,
            path,
            artifacts: Vec::new(),
            total_size: 0,
            last_modified: None,
        }
    }

    /// Add an artifact and update total size
    pub fn add_artifact(&mut self, artifact: Artifact) {
        self.total_size += artifact.size;
        self.artifacts.push(artifact);
    }

    /// Get human-readable total size
    pub fn size_display(&self) -> String {
        bytesize::ByteSize(self.total_size).to_string()
    }

    /// Age in whole days since last modification, relative to `now` (unix seconds).
    pub fn age_days(&self, now: i64) -> Option<i64> {
        self.last_modified.map(|lm| (now - lm).max(0) / 86_400)
    }

    /// Human-readable "time since last touched" string, e.g. "214 days ago".
    pub fn age_display(&self) -> String {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        match self.age_days(now) {
            Some(0) => "today".to_string(),
            Some(1) => "1 day ago".to_string(),
            Some(days) => format!("{} days ago", days),
            None => "unknown".to_string(),
        }
    }
}

/// Sort projects by cleanup priority: projects that are both large and long
/// untouched are ranked first. Combines a size rank and an age rank, each
/// normalized to 0..=1 relative to the given list, so both dimensions matter
/// regardless of their absolute scale (e.g. a huge fresh build next to a
/// tiny ancient one).
pub fn sort_by_cleanup_priority(projects: &mut [Project]) {
    if projects.len() < 2 {
        return;
    }

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);

    let max_size = projects.iter().map(|p| p.total_size).max().unwrap_or(0);
    let max_age = projects
        .iter()
        .filter_map(|p| p.age_days(now))
        .max()
        .unwrap_or(0);

    let score = |p: &Project| -> f64 {
        let size_score = if max_size > 0 {
            p.total_size as f64 / max_size as f64
        } else {
            0.0
        };
        let age_score = if max_age > 0 {
            p.age_days(now).unwrap_or(0) as f64 / max_age as f64
        } else {
            0.0
        };
        size_score + age_score
    };

    projects.sort_by(|a, b| {
        score(b)
            .partial_cmp(&score(a))
            .unwrap_or(std::cmp::Ordering::Equal)
    });
}

/// Represents a single artifact (directory or file) to be cleaned
#[derive(Debug, Clone)]
pub struct Artifact {
    /// Path to the artifact
    pub path: PathBuf,
    /// Size in bytes
    pub size: u64,
    /// Name of the artifact (directory/file name)
    pub name: String,
}

impl Artifact {
    /// Create a new artifact
    pub fn new(path: PathBuf, size: u64) -> Self {
        let name = path
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string());
        Self { path, size, name }
    }

    /// Get human-readable size
    pub fn size_display(&self) -> String {
        bytesize::ByteSize(self.size).to_string()
    }
}

/// Configuration derived from CLI arguments
#[derive(Debug, Clone)]
#[allow(dead_code)]  // Fields stored for future extensibility
pub struct Config {
    /// Root directory to scan
    pub root_dir: PathBuf,
    /// Dry-run mode (no actual deletion)
    pub dry_run: bool,
    /// Skip confirmation prompts
    pub auto_confirm: bool,
    /// Interactive selection mode
    pub interactive: bool,
    /// Additional patterns to include
    pub include_patterns: Vec<String>,
    /// Patterns to exclude
    pub exclude_patterns: Vec<String>,
    /// Filter by project types (empty = all)
    pub project_filters: Vec<ProjectType>,
    /// Minimum age in days (None = no filter)
    pub min_age_days: Option<u64>,
    /// Minimum size in bytes (None = no filter)
    pub min_size_bytes: Option<u64>,
    /// Verbose output
    pub verbose: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            root_dir: PathBuf::from("."),
            dry_run: false,
            auto_confirm: false,
            interactive: false,
            include_patterns: Vec::new(),
            exclude_patterns: Vec::new(),
            project_filters: Vec::new(),
            min_age_days: None,
            min_size_bytes: None,
            verbose: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn now_secs() -> i64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }

    fn make_project(name: &str, size: u64, age_days: i64) -> Project {
        let mut p = Project::new(name.to_string(), ProjectType::Rust, PathBuf::from(name));
        p.total_size = size;
        p.last_modified = Some(now_secs() - age_days * 86_400);
        p
    }

    #[test]
    fn test_sort_by_cleanup_priority_ranks_large_and_old_first() {
        let mut projects = vec![
            make_project("small_new", 10, 1),
            make_project("large_old", 1_000_000, 400),
            make_project("large_new", 1_000_000, 1),
            make_project("small_old", 10, 400),
        ];

        sort_by_cleanup_priority(&mut projects);

        assert_eq!(projects[0].name, "large_old");
        assert_eq!(projects.last().unwrap().name, "small_new");
    }

    #[test]
    fn test_sort_by_cleanup_priority_handles_missing_last_modified() {
        let mut projects = vec![
            make_project("known", 100, 30),
            Project::new("unknown".to_string(), ProjectType::Node, PathBuf::from("unknown")),
        ];

        // Should not panic even though one project has no last_modified.
        sort_by_cleanup_priority(&mut projects);
        assert_eq!(projects.len(), 2);
    }

    #[test]
    fn test_sort_by_cleanup_priority_noop_on_short_lists() {
        let mut single = vec![make_project("only", 5, 5)];
        sort_by_cleanup_priority(&mut single);
        assert_eq!(single.len(), 1);

        let mut empty: Vec<Project> = Vec::new();
        sort_by_cleanup_priority(&mut empty);
        assert!(empty.is_empty());
    }

    #[test]
    fn test_age_display_variants() {
        let mut p = Project::new("p".to_string(), ProjectType::Go, PathBuf::from("p"));
        assert_eq!(p.age_display(), "unknown");

        p.last_modified = Some(now_secs());
        assert_eq!(p.age_display(), "today");

        p.last_modified = Some(now_secs() - 86_400 * 5);
        assert_eq!(p.age_display(), "5 days ago");
    }
}
