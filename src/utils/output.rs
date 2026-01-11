use console::style;
use comfy_table::{presets::UTF8_FULL, Cell, ContentArrangement, Table};

use crate::Task;

/// Print a success message in green
pub fn success(msg: &str) {
    println!("{} {}", style("✓").green().bold(), msg);
}

/// Print an info message in blue
#[allow(dead_code)]
pub fn info(msg: &str) {
    println!("{} {}", style("ℹ").blue().bold(), msg);
}

/// Print an error message in red
#[allow(dead_code)]
pub fn error(msg: &str) {
    eprintln!("{} {}", style("✗").red().bold(), msg);
}

/// Print a warning message in yellow
#[allow(dead_code)]
pub fn warning(msg: &str) {
    println!("{} {}", style("⚠").yellow().bold(), msg);
}

/// Format a priority badge with color
pub fn priority_badge(priority: &str) -> String {
    let (symbol, color_fn): (&str, fn(String) -> console::StyledObject<String>) = match priority {
        "Critical" => ("🔴", |s| style(s).red().bold()),
        "High" => ("🟠", |s| style(s).yellow().bold()),
        "Medium" => ("🟡", |s| style(s).cyan()),
        "Low" => ("🟢", |s| style(s).green()),
        _ => ("⚪", |s| style(s).dim()),
    };

    format!("{} {}", symbol, color_fn(priority.to_string()))
}

/// Format a status badge with color
pub fn status_badge(status: &str) -> String {
    let color_fn: fn(String) -> console::StyledObject<String> = match status {
        "todo" => |s| style(s).cyan(),
        "in-progress" => |s| style(s).yellow().bold(),
        "testing" => |s| style(s).magenta(),
        "done" => |s| style(s).green(),
        _ => |s| style(s).white(),
    };

    color_fn(status.to_string()).to_string()
}

/// Format task ID in dim gray
pub fn task_id(id: &str) -> String {
    style(id).dim().to_string()
}

/// Format task slug in bold
pub fn task_slug(slug: &str) -> String {
    style(slug).bold().to_string()
}

/// Format tags in cyan
pub fn tags(tags: &[String]) -> String {
    let formatted: Vec<String> = tags.iter().map(|t| style(t).cyan().to_string()).collect();
    format!("({})", formatted.join(", "))
}

/// Get priority symbol and text without ANSI codes (for table display)
fn priority_for_table(priority: &str) -> String {
    let symbol = match priority {
        "Critical" => "🔴",
        "High" => "🟠",
        "Medium" => "🟡",
        "Low" => "🟢",
        _ => "⚪",
    };
    format!("{} {}", symbol, priority)
}

/// Format tasks as a table
pub fn format_tasks_as_table(tasks: &[Task]) -> String {
    let mut table = Table::new();

    // Set table style
    table.load_preset(UTF8_FULL);
    table.set_content_arrangement(ContentArrangement::Dynamic);

    // Add header
    table.set_header(vec!["Priority", "ID", "Title", "Tags"]);

    // Add rows
    for task in tasks {
        let task_priority = task.priority.as_deref().unwrap_or("Medium");
        let priority_display = priority_for_table(task_priority);

        let tags_display = task
            .tags
            .as_ref()
            .map(|tags| tags.join(", "))
            .unwrap_or_default();

        table.add_row(vec![
            Cell::new(priority_display),
            Cell::new(&task.id),
            Cell::new(&task.title),
            Cell::new(tags_display),
        ]);
    }

    table.to_string()
}
