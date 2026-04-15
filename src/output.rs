use crate::cache::{Cache, CacheCategory};
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
            "\n\u{1F4C1} {}\n",
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
    println!("\u{1F4CA} {}", "Summary".bold());
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
    println!("\n  \u{1F4C2} {}", "By ecosystem:".bold());
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
        println!("\n  \u{1F3C6} {}", "Largest projects:".bold());
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
        "  \u{1F4BE} {} {}",
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
            "\u{2728} {} Freed {} from {} project{}.",
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
    println!(
        "\n{} You can also run with {} to preview, or {} to generate a removal script.",
        "💡".yellow(),
        "--dry-run".cyan(),
        "--script cleanup.sh".cyan()
    );
}

// === Cache Display Functions ===

/// Print the list of found caches
pub fn print_caches(caches: &[Cache], dry_run: bool) {
    if caches.is_empty() {
        println!(
            "\n{} No user caches found to clean.",
            "\u{2139}".blue()
        );
        return;
    }

    if dry_run {
        println!(
            "\n\u{1F5C4} {}\n",
            "User caches found:".bold()
        );
    } else {
        println!(
            "\n\u{1F5C4} {}\n",
            "User caches to clean:".bold()
        );
    }

    for (i, cache) in caches.iter().enumerate() {
        print_cache_entry(i + 1, cache);
    }
}

/// Print a single cache entry
fn print_cache_entry(index: usize, cache: &Cache) {
    println!(
        "  {}. {} {} ({})",
        format!("{:>2}", index).dimmed(),
        cache.cache_type.icon(),
        cache.cache_type.label().cyan(),
        cache.size_display().yellow()
    );

    // Show warning if special handling needed
    if let Some(warning) = cache.cache_type.requires_app_closed() {
        println!(
            "       {} {}",
            "\u{26A0}".yellow(),
            warning.dimmed()
        );
    }
}

/// Print combined summary for projects and caches
pub fn print_combined_summary(projects: &[Project], caches: &[Cache]) {
    let project_size: u64 = projects.iter().map(|p| p.total_size).sum();
    let cache_size: u64 = caches.iter().map(|c| c.size).sum();
    let total_size = project_size + cache_size;
    let total_artifacts: usize = projects.iter().map(|p| p.artifacts.len()).sum();

    println!("\n{}", "─".repeat(60).dimmed());
    println!("\u{1F4CA} {}", "Summary".bold());
    println!("{}", "─".repeat(60).dimmed());

    // Project stats
    if !projects.is_empty() {
        println!(
            "  {} {} project{} with {} artifact director{}",
            "\u{2022}".cyan(),
            projects.len().to_string().white().bold(),
            if projects.len() == 1 { "" } else { "s" },
            total_artifacts.to_string().white().bold(),
            if total_artifacts == 1 { "y" } else { "ies" }
        );

        // Group by project type
        let mut by_type: HashMap<ProjectType, (usize, u64)> = HashMap::new();
        for project in projects {
            let entry = by_type.entry(project.project_type).or_insert((0, 0));
            entry.0 += 1;
            entry.1 += project.total_size;
        }
        let mut type_stats: Vec<_> = by_type.into_iter().collect();
        type_stats.sort_by(|a, b| b.1 .1.cmp(&a.1 .1));

        println!("\n  \u{1F4C2} {}", "By ecosystem:".bold());
        for (project_type, (count, size)) in &type_stats {
            println!(
                "     {} {:>3} {} {:>12}",
                project_type.icon(),
                count.to_string().white(),
                format!("{:<10}", project_type.label()).dimmed(),
                bytesize::ByteSize(*size).to_string().yellow()
            );
        }
    }

    // Cache stats
    if !caches.is_empty() {
        println!(
            "\n  {} {} user cache{}",
            "\u{2022}".cyan(),
            caches.len().to_string().white().bold(),
            if caches.len() == 1 { "" } else { "s" }
        );

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

        println!("\n  \u{1F5C4} {}", "User caches:".bold());
        for (category, (count, size)) in &cat_stats {
            println!(
                "     {} {:>3} {} {:>12}",
                category.icon(),
                count.to_string().white(),
                format!("{:<14}", category.label()).dimmed(),
                bytesize::ByteSize(*size).to_string().yellow()
            );
        }
    }

    // Combined largest items
    if !projects.is_empty() || !caches.is_empty() {
        // Create a unified list of (name, icon, size)
        let mut all_items: Vec<(String, &str, u64)> = Vec::new();

        for project in projects {
            all_items.push((
                project.name.clone(),
                project.project_type.icon(),
                project.total_size,
            ));
        }

        for cache in caches {
            all_items.push((
                cache.cache_type.label().to_string(),
                cache.cache_type.icon(),
                cache.size,
            ));
        }

        // Sort by size
        all_items.sort_by(|a, b| b.2.cmp(&a.2));

        let top_count = std::cmp::min(10, all_items.len());
        if top_count > 0 {
            println!("\n  \u{1F3C6} {}", "Largest items:".bold());
            for (i, (name, icon, size)) in all_items.iter().take(top_count).enumerate() {
                let rank = i + 1;
                let medal = match rank {
                    1 => "\u{1F947}",
                    2 => "\u{1F948}",
                    3 => "\u{1F949}",
                    _ => "  ",
                };
                println!(
                    "   {} {:>2}. {} {} {:>12}",
                    medal,
                    rank,
                    icon,
                    truncate_name(name, 30).cyan(),
                    bytesize::ByteSize(*size).to_string().yellow()
                );
            }

            let top_size: u64 = all_items.iter().take(top_count).map(|(_, _, s)| s).sum();
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
    }

    // Total
    println!("\n{}", "─".repeat(60).dimmed());
    println!(
        "  \u{1F4BE} {} {}",
        "Total space to reclaim:".bold(),
        bytesize::ByteSize(total_size).to_string().yellow().bold()
    );
    if !projects.is_empty() && !caches.is_empty() {
        println!(
            "       ({} projects + {} caches)",
            bytesize::ByteSize(project_size).to_string().dimmed(),
            bytesize::ByteSize(cache_size).to_string().dimmed()
        );
    }
    println!("{}", "─".repeat(60).dimmed());
}

/// Print script generation success message
pub fn print_script_generated(path: &std::path::Path, total_size: u64, item_count: usize) {
    println!(
        "\n\u{1F4DD} {} {}",
        "Script generated:".green().bold(),
        path.display().to_string().cyan()
    );
    println!(
        "  {} {} items, {} potential savings",
        "\u{2022}".dimmed(),
        item_count,
        bytesize::ByteSize(total_size).to_string().yellow()
    );
    println!(
        "  {} Review and edit the script, then run: {}",
        "\u{2022}".dimmed(),
        format!("chmod +x {} && {}", path.display(), path.display()).white()
    );
}
