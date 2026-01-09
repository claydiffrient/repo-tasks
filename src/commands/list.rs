use anyhow::{bail, Result};
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::{Config, Task};

/// List all tasks in a given status
pub fn list(status: Option<String>) -> Result<()> {
    // Check if initialized
    if !Config::is_initialized() {
        bail!("Not in a repo-tasks repository. Run 'tasks init' first.");
    }

    let config = Config::load()?;
    let status = status.unwrap_or_else(|| "todo".to_string());

    // Validate status
    if !config.statuses.contains(&status) {
        bail!("Invalid status '{}'. Valid statuses: {}", status, config.statuses.join(", "));
    }

    // Build path to status directory
    let tasks_dir = PathBuf::from(".repo-tasks/tasks").join(&status);

    if !tasks_dir.exists() {
        println!("No tasks in status '{}'", status);
        return Ok(());
    }

    // Collect tasks
    let mut tasks: Vec<Task> = WalkDir::new(&tasks_dir)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("md"))
        .filter_map(|e| Task::from_file(e.path()).ok())
        .collect();

    if tasks.is_empty() {
        println!("No tasks in status '{}'", status);
        return Ok(());
    }

    // Sort by priority (Critical > High > Medium > Low)
    tasks.sort_by(|a, b| {
        let priority_rank = |p: &Option<String>| {
            match p.as_deref() {
                Some("Critical") => 0,
                Some("High") => 1,
                Some("Medium") => 2,
                Some("Low") => 3,
                _ => 4,
            }
        };
        priority_rank(&a.priority).cmp(&priority_rank(&b.priority))
    });

    // Print tasks
    println!("Tasks in '{}' ({} total):", status, tasks.len());
    println!();

    for task in tasks {
        let priority = task.priority.as_deref().unwrap_or("Medium");
        let priority_symbol = match priority {
            "Critical" => "🔴",
            "High" => "🟠",
            "Medium" => "🟡",
            "Low" => "🟢",
            _ => "⚪",
        };

        println!("{} [{}] {} - {}", priority_symbol, task.id, task.slug, task.title);
    }

    Ok(())
}
