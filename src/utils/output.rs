use console::style;

/// Print a success message in green
pub fn success(msg: &str) {
    println!("{} {}", style("✓").green().bold(), msg);
}

/// Print an info message in blue
pub fn info(msg: &str) {
    println!("{} {}", style("ℹ").blue().bold(), msg);
}

/// Print an error message in red
pub fn error(msg: &str) {
    eprintln!("{} {}", style("✗").red().bold(), msg);
}

/// Print a warning message in yellow
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
