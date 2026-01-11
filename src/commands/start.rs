use anyhow::{bail, Context, Result};
use console::style;
use git2::Repository;
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::utils;
use crate::{Config, Task};

/// Start working on a task (move to in-progress + create git branch)
pub fn start(slug_or_id: String) -> Result<()> {
    // Check if initialized
    if !Config::is_initialized() {
        bail!("Not in a repo-tasks repository. Run 'tasks init' first.");
    }

    let _config = Config::load()?;

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

    let old_status = task.status.clone();
    let new_status = "in-progress".to_string();

    // Check if already in in-progress
    if task.status == new_status {
        println!(
            "{}",
            style("Task is already in-progress").yellow()
        );
    } else {
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
        println!();
    }

    // Create and checkout git branch
    let repo = Repository::open(".")
        .context("Not in a git repository. Initialize git with 'git init' first.")?;

    // Generate branch name: {id}-{slug}
    let branch_name = format!("{}-{}", task.id, task.slug);

    // Check if branch already exists
    let branch_exists = repo.find_branch(&branch_name, git2::BranchType::Local).is_ok();

    if branch_exists {
        // Checkout existing branch
        let obj = repo.revparse_single(&format!("refs/heads/{}", branch_name))?;
        repo.checkout_tree(&obj, None)?;
        repo.set_head(&format!("refs/heads/{}", branch_name))?;

        println!(
            "{} {}",
            style("Switched to existing branch:").cyan(),
            style(&branch_name).bold()
        );
    } else {
        // Create new branch from HEAD
        let head = repo.head()?;
        let commit = head.peel_to_commit()?;

        repo.branch(&branch_name, &commit, false)
            .context("Failed to create branch")?;

        // Checkout the new branch
        let obj = repo.revparse_single(&format!("refs/heads/{}", branch_name))?;
        repo.checkout_tree(&obj, None)?;
        repo.set_head(&format!("refs/heads/{}", branch_name))?;

        println!(
            "{} {}",
            style("Created and switched to new branch:").green(),
            style(&branch_name).bold()
        );
    }

    println!();
    println!("{}", style("Ready to start working!").dim());
    println!(
        "  {} {}",
        style("•").dim(),
        style("Make your changes and commit as usual").dim()
    );
    println!(
        "  {} {}",
        style("•").dim(),
        style("Run 'tasks save' to commit task file changes").dim()
    );

    Ok(())
}
