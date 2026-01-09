use anyhow::{bail, Result};
use console::style;
use dialoguer::{Input, MultiSelect, Select};
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::utils;
use crate::{Config, Task};

/// Update task properties
pub fn update(slug_or_id: String) -> Result<()> {
    // Check if initialized
    if !Config::is_initialized() {
        bail!("Not in a repo-tasks repository. Run 'tasks init' first.");
    }

    let config = Config::load()?;

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

    println!("Updating task: {}", task.title);
    println!();

    // Choose what to update
    let options = vec!["Title", "Priority", "Tags", "Body", "Done"];
    let selections = MultiSelect::new()
        .with_prompt("What would you like to update? (Space to select, Enter to confirm)")
        .items(&options)
        .interact()?;

    if selections.is_empty() {
        println!("No changes made.");
        return Ok(());
    }

    // Update selected fields
    for &index in &selections {
        match options[index] {
            "Title" => {
                let new_title: String = Input::new()
                    .with_prompt("New title")
                    .with_initial_text(&task.title)
                    .interact_text()?;

                if !new_title.trim().is_empty() {
                    task.title = new_title.trim().to_string();
                    task.slug = Task::generate_slug(&task.title);
                }
            }
            "Priority" => {
                let current_idx = task
                    .priority
                    .as_ref()
                    .and_then(|p| config.priorities.iter().position(|x| x == p))
                    .unwrap_or(1);

                let new_priority_idx = Select::new()
                    .with_prompt("New priority")
                    .items(&config.priorities)
                    .default(current_idx)
                    .interact()?;

                task.priority = Some(config.priorities[new_priority_idx].clone());
            }
            "Tags" => {
                let current_tags = task.tags.as_ref().map(|t| t.join(", ")).unwrap_or_default();

                let new_tags: String = Input::new()
                    .with_prompt("Tags (comma-separated)")
                    .with_initial_text(&current_tags)
                    .allow_empty(true)
                    .interact_text()?;

                if new_tags.trim().is_empty() {
                    task.tags = None;
                } else {
                    task.tags = Some(
                        new_tags
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect(),
                    );
                }
            }
            "Body" => {
                let new_body: String = Input::new()
                    .with_prompt("Description")
                    .with_initial_text(&task.body)
                    .allow_empty(true)
                    .interact_text()?;

                task.body = new_body;
            }
            "Done" => {
                // This is handled by selecting "Done" - we'll mark it for completion
                // but the actual move will be suggested
                println!("To mark as done, use 'tasks move {} done'", task.slug);
            }
            _ => {}
        }
    }

    // Determine new path (slug might have changed)
    let new_path = task.file_path(&task.status);

    // Save the task
    task.to_file(&new_path)?;

    // If path changed (due to slug change), remove old file
    if old_path != new_path {
        std::fs::remove_file(&old_path)?;
        utils::success(&format!("Updated task and renamed file"));
        println!("  Old: {}", style(old_path.display()).dim());
        println!("  New: {}", style(new_path.display()).dim());
    } else {
        utils::success(&format!("Updated task: {}", style(&task.slug).bold()));
    }

    Ok(())
}
