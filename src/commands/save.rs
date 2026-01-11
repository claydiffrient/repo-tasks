use anyhow::{bail, Context, Result};
use console;
use git2::{Repository, Signature, StatusOptions};

use crate::Config;

/// Save (commit) changes to the repo-tasks directory
pub fn save(message: Option<String>) -> Result<()> {
    // Check if initialized
    if !Config::is_initialized() {
        bail!("Not in a repo-tasks repository. Run 'tasks init' first.");
    }

    // Open the git repository
    let repo = Repository::open(".")
        .context("Not in a git repository. Initialize git with 'git init' first.")?;

    // Check if there are any changes in .repo-tasks/
    let mut status_opts = StatusOptions::new();
    status_opts.include_untracked(true);
    status_opts.pathspec(".repo-tasks/");

    let statuses = repo.statuses(Some(&mut status_opts))?;

    if statuses.is_empty() {
        println!("No changes to commit in .repo-tasks/");
        return Ok(());
    }

    // Show what will be committed
    println!("Changes to be committed:");
    for entry in statuses.iter() {
        let path = entry.path().unwrap_or("unknown");
        let status = entry.status();

        let status_str = if status.is_wt_new() {
            "new file"
        } else if status.is_wt_modified() {
            "modified"
        } else if status.is_wt_deleted() {
            "deleted"
        } else {
            "changed"
        };

        println!("  {}: {}", status_str, path);
    }
    println!();

    // Get or generate commit message
    let commit_message = message.unwrap_or_else(|| generate_commit_message(&repo, &statuses));

    // Check for staged files outside .repo-tasks/
    let index = repo.index()?;
    let head_commit = repo.head()?.peel_to_commit()?;
    let head_tree = head_commit.tree()?;
    let diff = repo.diff_tree_to_index(Some(&head_tree), Some(&index), None)?;

    let mut non_task_files = Vec::new();
    diff.foreach(
        &mut |delta, _| {
            if let Some(path) = delta.new_file().path() {
                let path_str = path.to_string_lossy();
                if !path_str.starts_with(".repo-tasks/") {
                    non_task_files.push(path_str.to_string());
                }
            }
            true
        },
        None,
        None,
        None,
    )?;

    if !non_task_files.is_empty() {
        eprintln!("\n{}", console::style("Error: Cannot commit non-task files with 'tasks save'").red().bold());
        eprintln!("\nThe following staged files are outside .repo-tasks/:");
        for file in &non_task_files {
            eprintln!("  - {}", console::style(file).yellow());
        }
        eprintln!("\n{}", console::style("To fix this:").bold());
        eprintln!("  1. Commit project files separately: {}", console::style("git commit -m \"Your message\"").cyan());
        eprintln!("  2. Then use 'tasks save' for task files only");
        eprintln!("\nOr unstage non-task files: {}", console::style("git restore --staged <file>").cyan());
        bail!("Staged files outside .repo-tasks/ directory");
    }

    // Stage .repo-tasks/ directory
    let mut index = repo.index()?;
    index.add_all([".repo-tasks/"].iter(), git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;

    // Create the commit
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    let parent_commit = repo.head()?.peel_to_commit()?;
    let sig = Signature::now("repo-tasks", "tasks@local")?;

    repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        &commit_message,
        &tree,
        &[&parent_commit],
    )?;

    println!("✓ Committed changes:");
    println!(
        "  {}",
        commit_message.lines().next().unwrap_or(&commit_message)
    );

    Ok(())
}

/// Generate a commit message based on the changes
fn generate_commit_message(_repo: &Repository, statuses: &git2::Statuses) -> String {
    let mut added = 0;
    let mut modified = 0;
    let mut deleted = 0;

    for entry in statuses.iter() {
        let status = entry.status();
        if status.is_wt_new() {
            added += 1;
        } else if status.is_wt_modified() {
            modified += 1;
        } else if status.is_wt_deleted() {
            deleted += 1;
        }
    }

    let mut parts = Vec::new();
    if added > 0 {
        parts.push(format!("{} added", added));
    }
    if modified > 0 {
        parts.push(format!("{} modified", modified));
    }
    if deleted > 0 {
        parts.push(format!("{} deleted", deleted));
    }

    if parts.is_empty() {
        "Update tasks".to_string()
    } else {
        format!("Update tasks: {}", parts.join(", "))
    }
}
