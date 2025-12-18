use crate::project::{Project, ProjectType};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;

/// Print the scanning spinner
pub fn scanning_spinner() -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("\u{25CF}\u{25D0}\u{25D1}\u{25D2}\u{25D3}\u{25D4}\u{25D5}\u{25D6} ")
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    spinner.set_message("Scanning for projects...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(80));
    spinner
}

/// Print the list of found projects
pub fn print_projects(projects: &[Project], dry_run: bool) {
    if projects.is_empty() {
        println!(
            "\n{} No build artifacts found to clean.",
            "\u{2139}".blue()
        );
        return;
    }

    let total_size: u64 = projects.iter().map(|p| p.total_size).sum();

    if dry_run {
        println!(
            "\n{} {} (dry-run mode - nothing will be deleted)\n",
            "\u{1F50D}".yellow(),
            "Preview of cleanable artifacts:".yellow().bold()
        );
    } else {
        println!(
            "\n{} {}\n",
            "\u{1F4C1}",
            "Found projects with cleanable artifacts:".bold()
        );
    }

    for (i, project) in projects.iter().enumerate() {
        print_project(i + 1, project);
    }

    // Print summary statistics
    print_scan_summary(projects, total_size);
}

/// Print comprehensive scan summary
fn print_scan_summary(projects: &[Project], total_size: u64) {
    // Count artifacts and group by type
    let total_artifacts: usize = projects.iter().map(|p| p.artifacts.len()).sum();

    // Group by project type
    let mut by_type: HashMap<ProjectType, (usize, u64)> = HashMap::new();
    for project in projects {
        let entry = by_type.entry(project.project_type).or_insert((0, 0));
        entry.0 += 1;
        entry.1 += project.total_size;
    }

    // Sort by size descending
    let mut type_stats: Vec<_> = by_type.into_iter().collect();
    type_stats.sort_by(|a, b| b.1 .1.cmp(&a.1 .1));

    println!("\n{}", "─".repeat(60).dimmed());
    println!("{} {}", "\u{1F4CA}", "Summary".bold());
    println!("{}", "─".repeat(60).dimmed());

    // Overall stats
    println!(
        "  {} {} project{} with {} artifact director{}",
        "\u{2022}".cyan(),
        projects.len().to_string().white().bold(),
        if projects.len() == 1 { "" } else { "s" },
        total_artifacts.to_string().white().bold(),
        if total_artifacts == 1 { "y" } else { "ies" }
    );

    // Breakdown by type
    println!("\n  {} {}", "\u{1F4C2}", "By ecosystem:".bold());
    for (project_type, (count, size)) in &type_stats {
        println!(
            "     {} {:>3} {} {:>12}",
            project_type.icon(),
            count.to_string().white(),
            format!("{:<10}", project_type.label()).dimmed(),
            bytesize::ByteSize(*size).to_string().yellow()
        );
    }

    // Top projects by size (show top 10 or fewer if less projects)
    let top_count = std::cmp::min(10, projects.len());
    if top_count > 0 {
        println!("\n  {} {}", "\u{1F3C6}", "Largest projects:".bold());
        for (i, project) in projects.iter().take(top_count).enumerate() {
            let rank = i + 1;
            let medal = match rank {
                1 => "\u{1F947}",  // gold
                2 => "\u{1F948}",  // silver
                3 => "\u{1F949}",  // bronze
                _ => "  ",
            };
            println!(
                "   {} {:>2}. {} {} {:>12}",
                medal,
                rank,
                project.project_type.icon(),
                truncate_name(&project.name, 30).cyan(),
                project.size_display().yellow()
            );
        }

        // Show how much the top projects represent
        let top_size: u64 = projects.iter().take(top_count).map(|p| p.total_size).sum();
        let percentage = if total_size > 0 {
            (top_size as f64 / total_size as f64 * 100.0) as u32
        } else {
            0
        };
        println!(
            "     {} Top {} = {} ({}% of total)",
            "\u{2192}".dimmed(),
            top_count,
            bytesize::ByteSize(top_size).to_string().yellow(),
            percentage.to_string().white()
        );
    }

    // Total
    println!("\n{}", "─".repeat(60).dimmed());
    println!(
        "  {} {} {}",
        "\u{1F4BE}",
        "Total space to reclaim:".bold(),
        bytesize::ByteSize(total_size).to_string().yellow().bold()
    );
    println!("{}", "─".repeat(60).dimmed());
}

/// Truncate a project name to fit display width
fn truncate_name(name: &str, max_len: usize) -> String {
    if name.len() <= max_len {
        format!("{:<width$}", name, width = max_len)
    } else {
        format!("{}...", &name[..max_len - 3])
    }
}

/// Print a single project entry
fn print_project(index: usize, project: &Project) {
    let artifacts_str: String = project
        .artifacts
        .iter()
        .map(|a| a.name.clone())
        .collect::<Vec<_>>()
        .join(", ");

    println!(
        "  {}. {} {} {} {} ({})",
        format!("{:>2}", index).dimmed(),
        project.project_type.icon(),
        project.name.cyan(),
        format!("({})", project.project_type.label()).dimmed(),
        format!("\u{2192} {}", artifacts_str).white(),
        project.size_display().yellow()
    );
}

/// Print verbose project details
pub fn print_project_verbose(project: &Project) {
    println!("\n{} {}", project.project_type.icon(), project.name.cyan());
    println!("   Path: {}", project.path.display().to_string().dimmed());
    println!(
        "   Type: {}",
        project.project_type.label().to_string().white()
    );
    println!("   Artifacts:");
    for artifact in &project.artifacts {
        println!(
            "     - {} ({})",
            artifact.path.display().to_string().white(),
            artifact.size_display().yellow()
        );
    }
    println!(
        "   Total: {}",
        project.size_display().yellow().bold()
    );
}

/// Print success message after deletion
pub fn print_deletion_success(project: &Project) {
    println!(
        "  {} Removed {} {} ({})",
        "\u{2714}".green(),
        project.project_type.icon(),
        project.name.cyan(),
        project.size_display().yellow()
    );
}

/// Print error message for failed deletion
pub fn print_deletion_error(project: &Project, error: &str) {
    println!(
        "  {} Failed to remove {}: {}",
        "\u{2718}".red(),
        project.name.cyan(),
        error.red()
    );
}

/// Print partial deletion warning
pub fn print_partial_deletion(project: &Project, deleted: u64, failed: usize) {
    println!(
        "  {} Partially removed {} ({} freed, {} artifacts failed)",
        "\u{26A0}".yellow(),
        project.name.cyan(),
        bytesize::ByteSize(deleted).to_string().yellow(),
        failed.to_string().red()
    );
}

/// Print the final summary
pub fn print_summary(deleted_count: usize, total_freed: u64, failed_count: usize) {
    println!();
    if deleted_count > 0 {
        println!(
            "{} {} Freed {} from {} project{}.",
            "\u{2728}",
            "Clean complete!".green().bold(),
            bytesize::ByteSize(total_freed).to_string().yellow().bold(),
            deleted_count,
            if deleted_count == 1 { "" } else { "s" }
        );
    }

    if failed_count > 0 {
        println!(
            "{} {} project{} could not be fully cleaned (check permissions).",
            "\u{26A0}".yellow(),
            failed_count,
            if failed_count == 1 { "" } else { "s" }
        );
    }

    if deleted_count == 0 && failed_count == 0 {
        println!("{} Nothing was deleted.", "\u{2139}".blue());
    }
}

/// Print an info message
pub fn print_info(msg: &str) {
    println!("{} {}", "\u{2139}".blue(), msg);
}

/// Print confirmation prompt message
pub fn print_confirm_prompt(count: usize, total_size: u64) {
    println!(
        "\n{} About to delete artifacts from {} project{} ({}).",
        "\u{2757}".red(),
        count,
        if count == 1 { "" } else { "s" },
        bytesize::ByteSize(total_size).to_string().yellow()
    );
}

/// Print abort message
pub fn print_aborted() {
    println!("\n{} Operation cancelled. No files were deleted.", "\u{1F6AB}".red());
}
