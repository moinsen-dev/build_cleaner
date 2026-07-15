use crate::cache::{Cache, CacheCategory};
use crate::cache_scanner::CacheScanner;
use crate::cli::{Args, CacheFilter, ProjectFilter};
use crate::output;
use crate::project::{sort_by_cleanup_priority, Config, Project, ProjectType};
use crate::scanner::Scanner;
use crate::script::ScriptGenerator;
use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, MultiSelect};
use std::fs;

/// Run the cleaner with the given arguments
pub fn run(args: Args) -> Result<()> {
    // Convert CLI args to Config
    let config = build_config(&args)?;

    // Show spinner while scanning
    let spinner = output::scanning_spinner();

    // Scan for projects (unless cache-only mode)
    let projects = if args.cache_only {
        Vec::new()
    } else {
        let scanner = Scanner::new(config.clone());
        scanner.scan()?
    };

    // Scan for user caches if requested
    let caches = if args.user_caches || args.cache_only {
        scan_user_caches(&args)?
    } else {
        Vec::new()
    };

    spinner.finish_and_clear();

    // Handle nothing found
    if projects.is_empty() && caches.is_empty() {
        output::print_info("No build artifacts or caches found to clean.");
        return Ok(());
    }

    // Script generation mode
    if let Some(script_path) = &args.script {
        return generate_script(&projects, &caches, script_path, args.dry_run);
    }

    // Print found items
    if args.verbose {
        for project in &projects {
            output::print_project_verbose(project);
        }
    } else if !projects.is_empty() && caches.is_empty() {
        // Only projects
        output::print_projects(&projects, args.dry_run);
    } else if projects.is_empty() && !caches.is_empty() {
        // Only caches
        output::print_caches(&caches, args.dry_run);
        print_cache_only_summary(&caches);
    } else {
        // Both projects and caches
        output::print_projects(&projects, args.dry_run);
        output::print_caches(&caches, args.dry_run);
        output::print_combined_summary(&projects, &caches);
    }

    // Dry-run mode: just show what would be deleted
    if args.dry_run {
        return Ok(());
    }

    // Interactive mode: let user select which items to clean
    let (projects_to_clean, caches_to_clean) = if args.interactive {
        (
            select_projects_interactive(&projects)?,
            select_caches_interactive(&caches)?,
        )
    } else {
        (projects, caches)
    };

    if projects_to_clean.is_empty() && caches_to_clean.is_empty() {
        output::print_info("No items selected for cleaning.");
        return Ok(());
    }

    // Calculate totals for confirmation
    let total_count = projects_to_clean.len() + caches_to_clean.len();
    let total_size: u64 = projects_to_clean.iter().map(|p| p.total_size).sum::<u64>()
        + caches_to_clean.iter().map(|c| c.size).sum::<u64>();

    // Confirmation prompt (unless --yes flag)
    if !args.yes && !confirm_deletion_combined(total_count, total_size)? {
        output::print_aborted();
        return Ok(());
    }

    // Perform deletion
    let (deleted_count, total_freed, failed_count) =
        delete_all(&projects_to_clean, &caches_to_clean)?;

    // Print summary
    output::print_summary(deleted_count, total_freed, failed_count);

    Ok(())
}

/// Scan for user caches with category filtering
fn scan_user_caches(args: &Args) -> Result<Vec<Cache>> {
    let scanner = match CacheScanner::new() {
        Some(s) => s,
        None => {
            output::print_info("Could not determine home directory, skipping user cache scan.");
            return Ok(Vec::new());
        }
    };

    // Convert CLI cache filters to CacheCategory
    let categories: Vec<CacheCategory> = args
        .cache_category
        .iter()
        .map(|f| match f {
            CacheFilter::AiMl => CacheCategory::AiMl,
            CacheFilter::Package => CacheCategory::PackageManager,
            CacheFilter::Dev => CacheCategory::Development,
            CacheFilter::Ide => CacheCategory::Ide,
            CacheFilter::Container => CacheCategory::Container,
        })
        .collect();

    let scanner = scanner
        .with_categories(categories)
        .with_min_size(args.min_size_bytes());

    Ok(scanner.scan())
}

/// Generate a cleanup script instead of deleting
fn generate_script(
    projects: &[Project],
    caches: &[Cache],
    script_path: &std::path::Path,
    dry_run: bool,
) -> Result<()> {
    let mut generator = ScriptGenerator::new();
    generator.add_projects(projects);
    generator.add_caches(caches);

    let total_size = generator.total_size();
    let item_count = generator.action_count();

    if dry_run {
        // Just preview
        println!("{}", generator.generate());
        output::print_info(&format!(
            "Dry-run: Script would be written to {}",
            script_path.display()
        ));
    } else {
        generator.write_to_file(script_path)?;
        output::print_script_generated(script_path, total_size, item_count);
    }

    Ok(())
}

/// Print summary for cache-only mode
fn print_cache_only_summary(caches: &[Cache]) {
    use colored::*;
    use std::collections::HashMap;

    let total_size: u64 = caches.iter().map(|c| c.size).sum();

    println!("\n{}", "─".repeat(60).dimmed());
    println!("\u{1F4CA} {}", "Summary".bold());
    println!("{}", "─".repeat(60).dimmed());

    // Group by category
    let mut by_category: HashMap<CacheCategory, (usize, u64)> = HashMap::new();
    for cache in caches {
        let entry = by_category
            .entry(cache.cache_type.category())
            .or_insert((0, 0));
        entry.0 += 1;
        entry.1 += cache.size;
    }
    let mut cat_stats: Vec<_> = by_category.into_iter().collect();
    cat_stats.sort_by(|a, b| b.1 .1.cmp(&a.1 .1));

    println!(
        "  {} {} user cache{}",
        "\u{2022}".cyan(),
        caches.len().to_string().white().bold(),
        if caches.len() == 1 { "" } else { "s" }
    );

    println!("\n  \u{1F5C4} {}", "By category:".bold());
    for (category, (count, size)) in &cat_stats {
        println!(
            "     {} {:>3} {} {:>12}",
            category.icon(),
            count.to_string().white(),
            format!("{:<14}", category.label()).dimmed(),
            bytesize::ByteSize(*size).to_string().yellow()
        );
    }

    println!("\n{}", "─".repeat(60).dimmed());
    println!(
        "  \u{1F4BE} {} {}",
        "Total space to reclaim:".bold(),
        bytesize::ByteSize(total_size).to_string().yellow().bold()
    );
    println!("{}", "─".repeat(60).dimmed());
}

/// Build Config from CLI Args
fn build_config(args: &Args) -> Result<Config> {
    let root_dir = args
        .directory
        .canonicalize()
        .unwrap_or_else(|_| args.directory.clone());

    // Convert ProjectFilter to ProjectType
    let project_filters: Vec<ProjectType> = args
        .lang
        .iter()
        .map(|f| match f {
            ProjectFilter::Node => ProjectType::Node,
            ProjectFilter::Rust => ProjectType::Rust,
            ProjectFilter::Python => ProjectType::Python,
            ProjectFilter::Flutter => ProjectType::Flutter,
            ProjectFilter::JavaMaven => ProjectType::JavaMaven,
            ProjectFilter::JavaGradle => ProjectType::JavaGradle,
            ProjectFilter::Cpp => ProjectType::Cpp,
            ProjectFilter::DotNet => ProjectType::DotNet,
            ProjectFilter::Go => ProjectType::Go,
        })
        .collect();

    Ok(Config {
        root_dir,
        dry_run: args.dry_run,
        auto_confirm: args.yes,
        interactive: args.interactive,
        include_patterns: args.include.clone(),
        exclude_patterns: args.exclude.clone(),
        project_filters,
        min_age_days: args.min_age,
        min_size_bytes: args.min_size_bytes(),
        verbose: args.verbose,
    })
}

/// Interactive project selection
fn select_projects_interactive(projects: &[Project]) -> Result<Vec<Project>> {
    if projects.is_empty() {
        return Ok(Vec::new());
    }

    // Rank by cleanup priority: projects that are both large and long
    // untouched come first, so the biggest wins are easy to spot and select.
    let mut ranked: Vec<Project> = projects.to_vec();
    sort_by_cleanup_priority(&mut ranked);

    let items: Vec<String> = ranked
        .iter()
        .map(|p| {
            format!(
                "{} {} ({}) - {} \u{2022} last touched {}",
                p.project_type.icon(),
                p.name,
                p.project_type.label(),
                p.size_display(),
                p.age_display()
            )
        })
        .collect();

    let defaults: Vec<bool> = vec![true; items.len()];

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt(
            "Select projects to clean (Space to toggle, Enter to confirm) \u{2014} oldest & largest first",
        )
        .items(&items)
        .defaults(&defaults)
        .interact()?;

    Ok(selections.into_iter().map(|i| ranked[i].clone()).collect())
}

/// Interactive cache selection
fn select_caches_interactive(caches: &[Cache]) -> Result<Vec<Cache>> {
    if caches.is_empty() {
        return Ok(Vec::new());
    }

    let items: Vec<String> = caches
        .iter()
        .map(|c| {
            let warning = if c.cache_type.requires_app_closed().is_some() {
                " [!]"
            } else {
                ""
            };
            format!(
                "{} {} - {}{}",
                c.cache_type.icon(),
                c.cache_type.label(),
                c.size_display(),
                warning
            )
        })
        .collect();

    let defaults: Vec<bool> = vec![true; items.len()];

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select caches to clean (Space to toggle, Enter to confirm)")
        .items(&items)
        .defaults(&defaults)
        .interact()?;

    Ok(selections.into_iter().map(|i| caches[i].clone()).collect())
}

/// Confirm deletion with combined count
fn confirm_deletion_combined(count: usize, total_size: u64) -> Result<bool> {
    output::print_confirm_prompt(count, total_size);

    let confirmed = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Proceed with deletion?")
        .default(false)
        .interact()?;

    Ok(confirmed)
}

/// Delete all selected projects and caches
fn delete_all(projects: &[Project], caches: &[Cache]) -> Result<(usize, u64, usize)> {
    let mut deleted_count = 0;
    let mut total_freed: u64 = 0;
    let mut failed_count = 0;

    println!("\nDeleting artifacts...\n");

    // Delete project artifacts
    for project in projects {
        match delete_project_artifacts(project) {
            Ok((freed, failed_artifacts)) => {
                if failed_artifacts == 0 {
                    output::print_deletion_success(project);
                    deleted_count += 1;
                } else {
                    output::print_partial_deletion(project, freed, failed_artifacts);
                    failed_count += 1;
                }
                total_freed += freed;
            }
            Err(e) => {
                output::print_deletion_error(project, &e.to_string());
                failed_count += 1;
            }
        }
    }

    // Delete caches
    for cache in caches {
        match delete_cache(cache) {
            Ok(freed) => {
                println!(
                    "  \u{2714} Removed {} {} ({})",
                    cache.cache_type.icon(),
                    cache.cache_type.label(),
                    bytesize::ByteSize(freed)
                );
                deleted_count += 1;
                total_freed += freed;
            }
            Err(e) => {
                println!(
                    "  \u{2718} Failed to remove {}: {}",
                    cache.cache_type.label(),
                    e
                );
                failed_count += 1;
            }
        }
    }

    Ok((deleted_count, total_freed, failed_count))
}

/// Delete artifacts for a single project
fn delete_project_artifacts(project: &Project) -> Result<(u64, usize)> {
    let mut freed: u64 = 0;
    let mut failed_count = 0;

    for artifact in &project.artifacts {
        match delete_artifact(&artifact.path) {
            Ok(()) => {
                freed += artifact.size;
            }
            Err(_) => {
                failed_count += 1;
            }
        }
    }

    Ok((freed, failed_count))
}

/// Delete a single cache
fn delete_cache(cache: &Cache) -> Result<u64> {
    delete_artifact(&cache.path)?;
    Ok(cache.size)
}

/// Delete a single artifact (directory or file)
fn delete_artifact(path: &std::path::Path) -> Result<()> {
    if path.is_dir() {
        fs::remove_dir_all(path)?;
    } else if path.is_file() {
        fs::remove_file(path)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::project::{Artifact, ProjectType};
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::TempDir;

    fn write_file(path: &std::path::Path, content: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        let mut f = File::create(path).unwrap();
        f.write_all(content.as_bytes()).unwrap();
    }

    fn create_project_with_artifact(tmp: &TempDir, artifact_name: &str) -> Project {
        let proj_path = tmp.path().join("test_project");
        fs::create_dir_all(&proj_path).unwrap();
        write_file(&proj_path.join("Cargo.toml"), "[package]");

        let artifact_path = proj_path.join(artifact_name);
        fs::create_dir_all(&artifact_path).unwrap();
        write_file(&artifact_path.join("file.txt"), "build output");

        let size = 12u64; // "build output" is 12 bytes
        let mut project = Project::new(
            "test_project".to_string(),
            ProjectType::Rust,
            proj_path,
        );
        project.add_artifact(Artifact::new(artifact_path, size));
        project
    }

    // --- delete_artifact tests ---

    #[test]
    fn test_delete_artifact_removes_directory() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path().join("to_delete");
        fs::create_dir_all(&dir).unwrap();
        write_file(&dir.join("file.txt"), "content");

        assert!(dir.exists());
        delete_artifact(&dir).unwrap();
        assert!(!dir.exists());
    }

    #[test]
    fn test_delete_artifact_removes_file() {
        let tmp = TempDir::new().unwrap();
        let file_path = tmp.path().join("artifact.bin");
        write_file(&file_path, "data");

        assert!(file_path.exists());
        delete_artifact(&file_path).unwrap();
        assert!(!file_path.exists());
    }

    #[test]
    fn test_delete_artifact_nonexistent_path_is_ok() {
        let tmp = TempDir::new().unwrap();
        let missing = tmp.path().join("does_not_exist");
        // Should succeed silently — nothing to delete
        delete_artifact(&missing).unwrap();
    }

    #[test]
    fn test_delete_artifact_removes_nested_contents() {
        let tmp = TempDir::new().unwrap();
        let dir = tmp.path().join("nested");
        let sub = dir.join("a").join("b").join("c");
        fs::create_dir_all(&sub).unwrap();
        write_file(&sub.join("deep.txt"), "deep content");

        delete_artifact(&dir).unwrap();
        assert!(!dir.exists());
    }

    // --- delete_project_artifacts tests ---

    #[test]
    fn test_delete_project_artifacts_frees_space() {
        let tmp = TempDir::new().unwrap();
        let project = create_project_with_artifact(&tmp, "target");

        let artifact_path = project.artifacts[0].path.clone();
        assert!(artifact_path.exists());

        let (freed, failed) = delete_project_artifacts(&project).unwrap();
        assert_eq!(failed, 0);
        assert!(freed > 0);
        assert!(!artifact_path.exists());
    }

    #[test]
    fn test_delete_project_artifacts_multiple_artifacts() {
        let tmp = TempDir::new().unwrap();
        let proj_path = tmp.path().join("multi_proj");
        fs::create_dir_all(&proj_path).unwrap();

        let mut project = Project::new(
            "multi_proj".to_string(),
            ProjectType::Node,
            proj_path.clone(),
        );

        for name in &["node_modules", "dist", ".next"] {
            let art_path = proj_path.join(name);
            fs::create_dir_all(&art_path).unwrap();
            write_file(&art_path.join("file.js"), "code");
            project.add_artifact(Artifact::new(art_path, 4));
        }

        let (freed, failed) = delete_project_artifacts(&project).unwrap();
        assert_eq!(failed, 0);
        assert_eq!(freed, 12); // 3 artifacts × 4 bytes each
    }

    #[test]
    fn test_delete_project_artifacts_empty_project_returns_zero() {
        let tmp = TempDir::new().unwrap();
        let proj_path = tmp.path().join("empty_proj");
        fs::create_dir_all(&proj_path).unwrap();

        let project = Project::new(
            "empty_proj".to_string(),
            ProjectType::Rust,
            proj_path,
        );

        let (freed, failed) = delete_project_artifacts(&project).unwrap();
        assert_eq!(freed, 0);
        assert_eq!(failed, 0);
    }

    // --- delete_cache tests ---

    #[test]
    fn test_delete_cache_removes_directory_and_returns_size() {
        use crate::cache::{Cache, CacheType};

        let tmp = TempDir::new().unwrap();
        let cache_dir = tmp.path().join("npm_cache");
        fs::create_dir_all(&cache_dir).unwrap();
        write_file(&cache_dir.join("pack.tgz"), "data");

        let reported_size = 42u64;
        let cache = Cache::new(CacheType::Npm, cache_dir.clone(), reported_size);

        assert!(cache_dir.exists());
        let freed = delete_cache(&cache).unwrap();
        assert_eq!(freed, reported_size);
        assert!(!cache_dir.exists());
    }
}
