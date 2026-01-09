use anyhow::{bail, Result};
use console::style;
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::utils;
use crate::{Config, Task};

/// Move a task to a different status
pub fn move_task(slug_or_id: String, new_status: String) -> Result<()> {
    // Check if initialized
    if !Config::is_initialized() {
        bail!("Not in a repo-tasks repository. Run 'tasks init' first.");
    }

    let config = Config::load()?;

    // Validate new status
    if !config.statuses.contains(&new_status) {
        bail!(
            "Invalid status '{}'. Valid statuses: {}",
            new_status,
            config.statuses.join(", ")
        );
    }

    // Find the task
    let tasks_base = PathBuf::from(".repo-tasks/tasks");
    let mut found_task: Option<(Task, PathBuf)> = None;

    for entry in WalkDir::new(&tasks_base)
        .min_depth(2)
        .max_depth(2)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("md"))
    {
        let filename = entry.file_name().to_string_lossy();

        if filename.contains(&slug_or_id) {
            if let Ok(task) = Task::from_file(entry.path()) {
                found_task = Some((task, entry.path().to_path_buf()));
                break;
            }
        }
    }

    let (mut task, old_path) = match found_task {
        Some(t) => t,
        None => bail!("Task not found: {}", slug_or_id),
    };

    // Check if already in target status
    if task.status == new_status {
        println!("Task is already in status '{}'", new_status);
        return Ok(());
    }

    let old_status = task.status.clone();

    // Update status
    task.status = new_status.clone();

    // Determine new path
    let new_path = task.file_path(&new_status);

    // Save to new location
    task.to_file(&new_path)?;

    // Remove from old location
    std::fs::remove_file(&old_path)?;

    utils::success(&format!("Moved task: {}", style(&task.title).bold()));
    println!(
        "  {} {} {} {}",
        style("From:").dim(),
        utils::status_badge(&old_status),
        style("→").dim(),
        utils::status_badge(&new_status)
    );
    println!("  {}", style(new_path.display()).dim());

    Ok(())
}
