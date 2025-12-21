use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Clone, ValueEnum, PartialEq, Eq)]
pub enum ProjectFilter {
    Node,
    Rust,
    Python,
    Flutter,
    JavaMaven,
    JavaGradle,
    Cpp,
    DotNet,
    Go,
}

#[derive(Debug, Clone, ValueEnum, PartialEq, Eq)]
pub enum CacheFilter {
    /// AI/ML model caches (Hugging Face, Ollama, PyTorch)
    AiMl,
    /// Package manager caches (npm, yarn, cargo, pip, gradle, maven)
    Package,
    /// Development tool caches (Xcode, Android SDK)
    Dev,
    /// IDE caches (JetBrains, VSCode)
    Ide,
    /// Container caches (Docker)
    Container,
}

#[derive(Parser, Debug)]
#[command(
    name = "build-cleaner",
    author,
    version,
    about = "Multi-language build directory cleaner - reclaim disk space by removing build artifacts",
    long_about = "A fast, safe tool to recursively find and delete build artifacts across multiple \
                  programming ecosystems (Node.js, Rust, Python, Flutter, Java, C/C++, .NET, Go). \
                  Supports dry-run mode, interactive selection, and configurable filters."
)]
pub struct Args {
    /// Directory to scan for projects (defaults to current directory)
    #[arg(default_value = ".")]
    pub directory: PathBuf,

    /// Preview mode - show what would be deleted without actually deleting
    #[arg(long, short = 'n')]
    pub dry_run: bool,

    /// Skip confirmation prompts and proceed with deletion
    #[arg(long, short = 'y')]
    pub yes: bool,

    /// Interactive selection mode - choose which projects to clean
    #[arg(long, short = 'i')]
    pub interactive: bool,

    /// Additional directory patterns to clean (can be repeated)
    #[arg(long, value_name = "PATTERN")]
    pub include: Vec<String>,

    /// Directory patterns to skip (can be repeated)
    #[arg(long, value_name = "PATTERN")]
    pub exclude: Vec<String>,

    /// Filter by project type (can be repeated for multiple types)
    #[arg(long, short = 'p', value_enum, value_name = "TYPE")]
    pub lang: Vec<ProjectFilter>,

    /// Only consider projects not modified in the last N days
    #[arg(long, value_name = "DAYS")]
    pub min_age: Option<u64>,

    /// Only consider artifacts above this size (e.g., "100MB", "1GB")
    #[arg(long, value_name = "SIZE")]
    pub min_size: Option<String>,

    /// Enable verbose output
    #[arg(long, short = 'v')]
    pub verbose: bool,

    // === User Cache Options ===
    /// Also scan user-level caches (AI models, package managers, IDE caches)
    #[arg(long, short = 'u')]
    pub user_caches: bool,

    /// Only scan user caches, skip project build artifacts
    #[arg(long)]
    pub cache_only: bool,

    /// Filter cache categories (can be repeated: ai-ml, package, dev, ide, container)
    #[arg(long, value_enum, value_name = "CATEGORY")]
    pub cache_category: Vec<CacheFilter>,

    // === Script Generation ===
    /// Generate a shell script instead of deleting directly
    #[arg(long, value_name = "PATH")]
    pub script: Option<PathBuf>,

    // === Web UI Options ===
    /// Launch web UI dashboard instead of terminal output
    #[arg(long)]
    pub serve: bool,

    /// Port for web UI server (default: 8080)
    #[arg(long, default_value = "8080")]
    pub port: u16,

    /// Don't automatically open browser when starting web UI
    #[arg(long)]
    pub no_open: bool,
}

impl Args {
    /// Parse the min_size argument into bytes
    pub fn min_size_bytes(&self) -> Option<u64> {
        self.min_size.as_ref().and_then(|s| parse_size(s))
    }
}

/// Parse human-readable size string into bytes
fn parse_size(s: &str) -> Option<u64> {
    let s = s.trim().to_uppercase();

    // Try to find where the number ends and unit begins
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_size() {
        assert_eq!(parse_size("100"), Some(100));
        assert_eq!(parse_size("100B"), Some(100));
        assert_eq!(parse_size("1KB"), Some(1_000));
        assert_eq!(parse_size("1KiB"), Some(1_024));
        assert_eq!(parse_size("100MB"), Some(100_000_000));
        assert_eq!(parse_size("1GB"), Some(1_000_000_000));
        assert_eq!(parse_size("1GiB"), Some(1_073_741_824));
        assert_eq!(parse_size("invalid"), None);
    }
}
