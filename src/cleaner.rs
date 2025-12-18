use crate::cli::{Args, ProjectFilter};
use crate::output;
use crate::project::{Config, Project, ProjectType};
use crate::scanner::Scanner;
use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, MultiSelect};
use std::fs;

/// Run the cleaner with the given arguments
pub fn run(args: Args) -> Result<()> {
    // Convert CLI args to Config
    let config = build_config(&args)?;

    // Show spinner while scanning
    let spinner = output::scanning_spinner();

    // Scan for projects
    let scanner = Scanner::new(config.clone());
    let projects = scanner.scan()?;

    spinner.finish_and_clear();

    // Handle no projects found
    if projects.is_empty() {
        output::print_projects(&projects, args.dry_run);
        return Ok(());
    }

    // Print found projects
    if args.verbose {
        for project in &projects {
            output::print_project_verbose(project);
        }
    } else {
        output::print_projects(&projects, args.dry_run);
    }

    // Dry-run mode: just show what would be deleted
    if args.dry_run {
        return Ok(());
    }

    // Interactive mode: let user select which projects to clean
    let projects_to_clean = if args.interactive {
        select_projects_interactive(&projects)?
    } else {
        projects
    };

    if projects_to_clean.is_empty() {
        output::print_info("No projects selected for cleaning.");
        return Ok(());
    }

    // Confirmation prompt (unless --yes flag)
    if !args.yes && !confirm_deletion(&projects_to_clean)? {
        output::print_aborted();
        return Ok(());
    }

    // Perform deletion
    let (deleted_count, total_freed, failed_count) = delete_projects(&projects_to_clean)?;

    // Print summary
    output::print_summary(deleted_count, total_freed, failed_count);

    Ok(())
}

/// Build Config from CLI Args
fn build_config(args: &Args) -> Result<Config> {
    let root_dir = args.directory.canonicalize().unwrap_or_else(|_| args.directory.clone());

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
    let items: Vec<String> = projects
        .iter()
        .map(|p| {
            format!(
                "{} {} ({}) - {}",
                p.project_type.icon(),
                p.name,
                p.project_type.label(),
                p.size_display()
            )
        })
        .collect();

    let defaults: Vec<bool> = vec![true; items.len()];

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select projects to clean (Space to toggle, Enter to confirm)")
        .items(&items)
        .defaults(&defaults)
        .interact()?;

    Ok(selections
        .into_iter()
        .map(|i| projects[i].clone())
        .collect())
}

/// Confirm deletion with user
fn confirm_deletion(projects: &[Project]) -> Result<bool> {
    let total_size: u64 = projects.iter().map(|p| p.total_size).sum();
    output::print_confirm_prompt(projects.len(), total_size);

    let confirmed = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Proceed with deletion?")
        .default(false)
        .interact()?;

    Ok(confirmed)
}

/// Delete the artifacts for the given projects
fn delete_projects(projects: &[Project]) -> Result<(usize, u64, usize)> {
    let mut deleted_count = 0;
    let mut total_freed: u64 = 0;
    let mut failed_count = 0;

    println!("\nDeleting artifacts...\n");

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

/// Delete a single artifact (directory or file)
fn delete_artifact(path: &std::path::Path) -> Result<()> {
    if path.is_dir() {
        fs::remove_dir_all(path)?;
    } else if path.is_file() {
        fs::remove_file(path)?;
    }
    Ok(())
}
