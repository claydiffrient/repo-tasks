use anyhow::{bail, Result};
use console::style;
use dialoguer::{Input, Select};

use crate::utils;
use crate::{Config, Task};

/// Create a new task interactively
pub fn new() -> Result<()> {
    // Check if initialized
    if !Config::is_initialized() {
        bail!("Not in a repo-tasks repository. Run 'tasks init' first.");
    }

    let config = Config::load()?;

    // Prompt for title
    let title: String = Input::new().with_prompt("Task title").interact_text()?;

    if title.trim().is_empty() {
        bail!("Task title cannot be empty");
    }

    // Prompt for priority
    let priority_index = Select::new()
        .with_prompt("Priority")
        .items(&config.priorities)
        .default(1) // Default to "Medium" (index 1)
        .interact()?;

    let priority = config.priorities[priority_index].clone();

    // Create task
    let task = Task::new(title.trim().to_string(), priority);
    let path = task.file_path("todo");

    // Save task
    task.to_file(&path)?;

    utils::success(&format!("Created task: {}", style(&task.slug).bold()));
    println!("  ID: {}", style(&task.id).dim());
    println!(
        "  Priority: {}",
        utils::priority_badge(task.priority.as_ref().unwrap())
    );
    println!("  File: {}", style(path.display()).dim());

    Ok(())
}
