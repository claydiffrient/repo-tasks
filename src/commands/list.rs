use anyhow::{bail, Result};
use console::style;
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::utils;
use crate::{Config, Task};

/// List all tasks in a given status
pub fn list(status: Option<String>, priority: Option<String>, tag: Option<String>) -> Result<()> {
    // Check if initialized
    if !Config::is_initialized() {
        bail!("Not in a repo-tasks repository. Run 'tasks init' first.");
    }

    let config = Config::load()?;
    let status = status.unwrap_or_else(|| "todo".to_string());

    // Validate status
    if !config.statuses.contains(&status) {
        bail!(
            "Invalid status '{}'. Valid statuses: {}",
            status,
            config.statuses.join(", ")
        );
    }

    // Validate priority if provided
    if let Some(ref p) = priority {
        if !config.priorities.contains(p) {
            bail!(
                "Invalid priority '{}'. Valid priorities: {}",
                p,
                config.priorities.join(", ")
            );
        }
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

    // Apply filters
    if let Some(ref filter_priority) = priority {
        tasks.retain(|t| t.priority.as_ref() == Some(filter_priority));
    }

    if let Some(ref filter_tag) = tag {
        tasks.retain(|t| {
            t.tags
                .as_ref()
                .map(|tags| tags.iter().any(|t| t == filter_tag))
                .unwrap_or(false)
        });
    }

    if tasks.is_empty() {
        let mut msg = format!("No tasks in status '{}'", status);
        if priority.is_some() || tag.is_some() {
            msg.push_str(" matching filters");
        }
        println!("{}", msg);
        return Ok(());
    }

    // Sort by priority (Critical > High > Medium > Low)
    tasks.sort_by(|a, b| {
        let priority_rank = |p: &Option<String>| match p.as_deref() {
            Some("Critical") => 0,
            Some("High") => 1,
            Some("Medium") => 2,
            Some("Low") => 3,
            _ => 4,
        };
        priority_rank(&a.priority).cmp(&priority_rank(&b.priority))
    });

    // Print header
    let mut header = format!("Tasks in {}", style(&status).bold().cyan());
    header.push_str(&format!(" ({} total)", style(tasks.len()).bold()));

    if let Some(ref p) = priority {
        header.push_str(&format!(" [priority: {}]", style(p).yellow()));
    }
    if let Some(ref t) = tag {
        header.push_str(&format!(" [tag: {}]", style(t).cyan()));
    }
    println!("{}", header);
    println!();

    // Print tasks
    for task in tasks {
        let task_priority = task.priority.as_deref().unwrap_or("Medium");
        let priority_display = utils::priority_badge(task_priority);
        let id_display = utils::task_id(&task.id);
        let slug_display = utils::task_slug(&task.slug);

        print!(
            "{} [{}] {} - {}",
            priority_display, id_display, slug_display, task.title
        );

        // Show tags if present
        if let Some(task_tags) = &task.tags {
            if !task_tags.is_empty() {
                print!(" {}", utils::tags(task_tags));
            }
        }

        println!();
    }

    Ok(())
}
