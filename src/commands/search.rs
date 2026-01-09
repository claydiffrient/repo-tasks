use anyhow::{bail, Result};
use grep_regex::RegexMatcher;
use grep_searcher::sinks::UTF8;
use grep_searcher::Searcher;
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::Config;

/// Search for tasks containing a query string
pub fn search(query: String) -> Result<()> {
    // Check if initialized
    if !Config::is_initialized() {
        bail!("Not in a repo-tasks repository. Run 'tasks init' first.");
    }

    let tasks_base = PathBuf::from(".repo-tasks/tasks");

    if !tasks_base.exists() {
        println!("No tasks found.");
        return Ok(());
    }

    // Create regex matcher
    let matcher = RegexMatcher::new_line_matcher(&query)?;
    let mut searcher = Searcher::new();

    // Collect all task files
    let task_files: Vec<PathBuf> = WalkDir::new(&tasks_base)
        .min_depth(2)
        .max_depth(2)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("md"))
        .map(|e| e.path().to_path_buf())
        .collect();

    if task_files.is_empty() {
        println!("No tasks found.");
        return Ok(());
    }

    let mut found_matches = false;

    // Search each file
    for path in task_files {
        let path_clone = path.clone();

        let result = searcher.search_path(
            &matcher,
            &path,
            UTF8(|lnum, line| {
                if !found_matches {
                    println!("Search results for '{}':", query);
                    println!();
                    found_matches = true;
                }

                // Extract task slug from filename
                let filename = path_clone.file_name()
                    .and_then(|f| f.to_str())
                    .unwrap_or("unknown");

                let slug = if let Some(slug_part) = filename.strip_suffix(".md") {
                    if let Some((_, slug)) = slug_part.split_once('-') {
                        slug
                    } else {
                        filename
                    }
                } else {
                    filename
                };

                println!("  {} [line {}]:", slug, lnum);
                println!("    {}", line.trim());
                println!();

                Ok(true)
            }),
        );

        if let Err(e) = result {
            eprintln!("Warning: Failed to search {}: {}", path.display(), e);
        }
    }

    if !found_matches {
        println!("No matches found for '{}'", query);
    }

    Ok(())
}
