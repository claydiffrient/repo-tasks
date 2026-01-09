use anyhow::{bail, Result};
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::{Config, Task};

/// Show details of a specific task
pub fn show(slug_or_id: String) -> Result<()> {
    // Check if initialized
    if !Config::is_initialized() {
        bail!("Not in a repo-tasks repository. Run 'tasks init' first.");
    }

    // Search for task across all status directories
    let tasks_base = PathBuf::from(".repo-tasks/tasks");

    let mut found_task: Option<Task> = None;

    for entry in WalkDir::new(&tasks_base)
        .min_depth(2)
        .max_depth(2)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("md"))
    {
        let filename = entry.file_name().to_string_lossy();

        // Check if filename contains the slug or ID
        if filename.contains(&slug_or_id) {
            if let Ok(task) = Task::from_file(entry.path()) {
                found_task = Some(task);
                break;
            }
        }
    }

    match found_task {
        Some(task) => {
            println!("Task: {}", task.title);
            println!("ID: {}", task.id);
            println!("Slug: {}", task.slug);
            println!("Status: {}", task.status);

            if let Some(priority) = &task.priority {
                println!("Priority: {}", priority);
            }

            if let Some(tags) = &task.tags {
                if !tags.is_empty() {
                    println!("Tags: {}", tags.join(", "));
                }
            }

            if let Some(blocks) = &task.blocks {
                if !blocks.is_empty() {
                    println!("Blocks: {}", blocks.join(", "));
                }
            }

            if let Some(depends_on) = &task.depends_on {
                if !depends_on.is_empty() {
                    println!("Depends on: {}", depends_on.join(", "));
                }
            }

            if !task.body.is_empty() {
                println!("\nDescription:");
                println!("{}", task.body);
            }

            Ok(())
        }
        None => {
            bail!("Task not found: {}", slug_or_id);
        }
    }
}
