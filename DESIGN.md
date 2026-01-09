# Design Document: repo-tasks CLI

## Executive Summary

`repo-tasks` is a lightweight, file-based task management system designed for git repositories with a primary focus on LLM-friendliness and CLI performance. This document outlines the technical design, tooling recommendations, and implementation strategy.

## Design Principles

1. **Performance First**: Sub-100ms response times for all commands except search
2. **Simple Data Model**: Plain Markdown files with YAML frontmatter—no database overhead
3. **LLM-Native**: Structure optimized for AI agent manipulation
4. **Git-Integrated**: Native git operations without abstractions
5. **Zero Configuration**: Sensible defaults, minimal setup required

## Tooling Recommendations

### Language: Rust

**Rationale:**
- **Startup Performance**: Sub-millisecond startup time when compiled
- **Execution Speed**: Near-C performance for file I/O operations
- **Memory Safety**: No runtime overhead, no garbage collection pauses
- **Rich Ecosystem**: Excellent CLI libraries (clap, serde, etc.)
- **Cross-Platform**: Single binary distribution, no runtime dependencies

**Alternative Considered:** Go
- Pros: Simpler syntax, faster compilation, good CLI ecosystem
- Cons: GC pauses (albeit small), slightly slower single-threaded performance
- Verdict: Rust preferred for maximum performance

### Core Dependencies

#### CLI Framework: `clap` (v4+)
```rust
// Modern derive API, compile-time validation
#[derive(Parser)]
#[command(name = "tasks")]
#[command(about = "Fast task management for git repositories")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
```

**Benefits:**
- Compile-time argument validation
- Auto-generated help/completion
- Minimal runtime overhead
- Excellent error messages

#### Configuration & Serialization: `serde` + `serde_yaml`
```rust
#[derive(Deserialize, Serialize)]
struct Config {
    project_name: String,
    statuses: Vec<String>,
    priorities: Vec<String>,
    // ...
}
```

**Benefits:**
- Zero-copy deserialization where possible
- Compile-time schema validation
- Excellent error messages for malformed YAML

#### File System Operations: `std::fs` + `walkdir`
- Use standard library for direct operations (fastest)
- `walkdir` for efficient directory traversal
- Avoid heavy async overhead for simple file ops

#### Search: `grep-rs` / `ripgrep` crate
```rust
// Leverage ripgrep's optimized search algorithms
use grep_searcher::Searcher;
use grep_matcher::Matcher;
```

**Benefits:**
- Fastest regex search available
- Same engine as `rg` command
- Multi-threaded by default
- Respects `.gitignore` automatically

#### Git Integration: `git2-rs` (libgit2 bindings)
```rust
let repo = Repository::open(".")?;
let mut index = repo.index()?;
index.add_path(Path::new(".repo-tasks"))?;
```

**Benefits:**
- Native git operations without shelling out
- Full git API access
- Much faster than spawning `git` processes

#### Terminal Output: `console` + `indicatif`
- `console`: Cross-platform terminal styling
- `indicatif`: Progress bars for long operations
- Keep output minimal for speed

### Alternative Architecture: Shell Script (Not Recommended)

**Pros:**
- Quick to prototype
- No compilation needed
- Easy to modify

**Cons:**
- Slow startup (shell parsing, sourcing)
- Poor error handling
- Platform dependencies
- 10-100x slower than compiled solution

**Verdict:** Not suitable for performance requirements.

## Architecture

### Directory Structure
```
repo-tasks/
├── src/
│   ├── main.rs           # Entry point, CLI setup
│   ├── commands/         # Command implementations
│   │   ├── mod.rs
│   │   ├── init.rs
│   │   ├── new.rs
│   │   ├── list.rs
│   │   ├── show.rs
│   │   ├── update.rs
│   │   ├── move_task.rs
│   │   ├── open.rs
│   │   ├── search.rs
│   │   └── save.rs
│   ├── models/           # Data structures
│   │   ├── mod.rs
│   │   ├── task.rs       # Task struct with frontmatter
│   │   ├── config.rs     # Config struct
│   │   └── status.rs     # Status enum
│   ├── utils/            # Shared utilities
│   │   ├── mod.rs
│   │   ├── fs.rs         # File system helpers
│   │   ├── git.rs        # Git operations
│   │   ├── slugify.rs    # Slug generation
│   │   └── frontmatter.rs # YAML parsing
│   └── lib.rs            # Library exports
├── tests/                # Integration tests
├── Cargo.toml
└── Cargo.lock
```

### Core Data Structures

```rust
// models/task.rs
#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    #[serde(rename = "ID")]
    pub id: String,

    #[serde(rename = "Title")]
    pub title: String,

    #[serde(rename = "Priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,

    #[serde(rename = "Blocks", skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<String>>,

    #[serde(rename = "DependsOn", skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<String>>,

    #[serde(rename = "Tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,

    #[serde(skip)]
    pub body: String,

    #[serde(skip)]
    pub slug: String,

    #[serde(skip)]
    pub status: String,
}

impl Task {
    pub fn from_file(path: &Path) -> Result<Self>;
    pub fn to_file(&self, path: &Path) -> Result<()>;
    pub fn parse_frontmatter(content: &str) -> Result<(Self, String)>;
    pub fn generate_slug(title: &str) -> String;
    pub fn generate_id() -> String;
}
```

### Performance Optimizations

#### 1. Lazy Loading
- Don't parse task bodies for `list` command
- Only read frontmatter (stop at `---` delimiter)
- Stream results rather than loading all into memory

```rust
// Fast list implementation
pub fn list_tasks(status: &str) -> Result<Vec<TaskSummary>> {
    let tasks_dir = PathBuf::from(".repo-tasks/tasks").join(status);

    WalkDir::new(tasks_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension() == Some("md"))
        .map(|e| TaskSummary::from_frontmatter_only(e.path()))
        .collect()
}
```

#### 2. Parallel Operations (where beneficial)
- Use `rayon` for parallel search across multiple files
- Keep simple operations single-threaded (overhead not worth it)

```rust
use rayon::prelude::*;

pub fn search_tasks(query: &str) -> Result<Vec<SearchResult>> {
    find_all_task_files()?
        .par_iter()  // Parallel iterator
        .filter_map(|path| search_file(path, query).ok())
        .flatten()
        .collect()
}
```

#### 3. Caching (Optional, Phase 2)
- Cache parsed config in memory during command execution
- Don't persist cache to disk (adds complexity, minimal benefit)

#### 4. Avoid Allocations
```rust
// Use string slices where possible
pub fn parse_slug(filename: &str) -> &str {
    filename
        .strip_suffix(".md")
        .and_then(|s| s.split_once('-'))
        .map(|(_, slug)| slug)
        .unwrap_or(filename)
}
```

### Error Handling Strategy

#### Use `anyhow` for Application Errors
```rust
use anyhow::{Context, Result};

pub fn load_config() -> Result<Config> {
    let content = fs::read_to_string(".repo-tasks/config.json")
        .context("Failed to read config.json")?;

    serde_json::from_str(&content)
        .context("Invalid JSON in config.json")
}
```

**Benefits:**
- User-friendly error messages
- Error chain context
- Minimal boilerplate

#### Exit Codes
- 0: Success
- 1: General error
- 2: Invalid arguments
- 3: Not in a repo-tasks repository

## Command Implementation Details

### `tasks init`

**Performance Target:** < 50ms

**Implementation:**
1. Check if `.repo-tasks/` exists (fail if yes)
2. Create directory structure
3. Write default `config.json`
4. Optional: Initialize git if not in a repo

```rust
pub fn init(project_name: Option<String>) -> Result<()> {
    let base = Path::new(".repo-tasks");

    if base.exists() {
        bail!("Repository already initialized");
    }

    // Create all directories
    fs::create_dir_all(base.join("tasks/todo"))?;
    fs::create_dir_all(base.join("tasks/in-progress"))?;
    fs::create_dir_all(base.join("tasks/testing"))?;
    fs::create_dir_all(base.join("tasks/done"))?;

    // Write config
    let config = Config::default(project_name);
    config.write()?;

    println!("✓ Initialized repo-tasks in {}", base.display());
    Ok(())
}
```

### `tasks new`

**Performance Target:** < 100ms (includes editor open time)

**Implementation:**
1. Generate ID (timestamp)
2. Prompt for title (using `dialoguer` for nice UX)
3. Generate slug from title
4. Create file with template
5. Open in editor if desired
6. Auto-commit if configured

```rust
use dialoguer::{Input, Select};

pub fn new() -> Result<()> {
    let config = Config::load()?;

    let title: String = Input::new()
        .with_prompt("Task title")
        .interact_text()?;

    let priority = Select::new()
        .with_prompt("Priority")
        .items(&config.priorities)
        .default(2)  // Medium
        .interact()?;

    let task = Task::new(title, config.priorities[priority].clone());
    let path = task.file_path("todo");

    task.to_file(&path)?;

    if config.auto_commit {
        git_commit(&format!("tasks: created {}", task.slug))?;
    }

    println!("✓ Created task: {}", task.slug);
    Ok(())
}
```

### `tasks list`

**Performance Target:** < 50ms for 1000 tasks

**Implementation:**
- Stream file reads
- Parse only frontmatter
- Sort by priority (in-memory, fast)
- Format output in single pass

```rust
pub fn list(opts: ListOptions) -> Result<()> {
    let tasks = load_tasks(&opts)?;

    // Sort by priority
    let mut tasks = tasks;
    tasks.sort_by_key(|t| priority_rank(&t.priority));

    // Format output
    for task in tasks {
        println!("{} [{}] {}",
            task.slug,
            task.priority.unwrap_or("Medium"),
            task.title
        );
    }

    Ok(())
}
```

### `tasks search`

**Performance Target:** < 200ms for 1000 tasks

**Implementation:**
- Use ripgrep library for multi-threaded search
- Search across all files in `.repo-tasks/tasks/`
- Highlight matches in terminal output

```rust
use grep_searcher::{sinks::UTF8, Searcher};
use grep_regex::RegexMatcher;

pub fn search(query: &str) -> Result<()> {
    let matcher = RegexMatcher::new_line_matcher(query)?;
    let mut searcher = Searcher::new();

    for path in find_all_task_files()? {
        searcher.search_path(
            &matcher,
            &path,
            UTF8(|lnum, line| {
                println!("{}:{}: {}", path.display(), lnum, line);
                Ok(true)
            })
        )?;
    }

    Ok(())
}
```

### `tasks save`

**Performance Target:** < 100ms (git operations dominate)

**Implementation:**
1. Stage `.repo-tasks/` directory
2. Generate commit message if not provided
3. Commit with git2

```rust
pub fn save(message: Option<String>) -> Result<()> {
    let repo = Repository::open(".")?;
    let mut index = repo.index()?;

    // Stage .repo-tasks/
    index.add_path(Path::new(".repo-tasks"))?;
    index.write()?;

    // Determine commit message
    let msg = message.unwrap_or_else(|| generate_commit_message(&repo));

    // Create commit
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let parent = repo.head()?.peel_to_commit()?;
    let sig = repo.signature()?;

    repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        &msg,
        &tree,
        &[&parent],
    )?;

    println!("✓ Committed: {}", msg);
    Ok(())
}
```

## Build & Distribution

### Compilation Flags for Maximum Performance

```toml
# Cargo.toml
[profile.release]
opt-level = 3          # Maximum optimization
lto = true             # Link-time optimization
codegen-units = 1      # Better optimization, slower compile
strip = true           # Strip symbols
panic = 'abort'        # Smaller binary
```

### Target Platforms
- macOS (aarch64-apple-darwin, x86_64-apple-darwin)
- Linux (x86_64-unknown-linux-gnu, x86_64-unknown-linux-musl)
- Windows (x86_64-pc-windows-msvc)

### Distribution
- GitHub Releases with pre-built binaries
- Homebrew tap for macOS
- Cargo install from crates.io
- Binary size target: < 5MB (stripped release build)

## Testing Strategy

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slug_generation() {
        assert_eq!(
            Task::generate_slug("Create Requirements Document"),
            "create-requirements-document"
        );
    }
}
```

### Integration Tests
```rust
// tests/cli_tests.rs
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_init_creates_directory() {
    let temp = tempdir()?;

    Command::cargo_bin("tasks")?
        .current_dir(&temp)
        .arg("init")
        .assert()
        .success();

    assert!(temp.path().join(".repo-tasks/config.json").exists());
}
```

### Performance Benchmarks
```rust
// benches/list_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_list(c: &mut Criterion) {
    c.bench_function("list 1000 tasks", |b| {
        b.iter(|| list_tasks(black_box("todo")))
    });
}

criterion_group!(benches, benchmark_list);
criterion_main!(benches);
```

## Implementation Phases

### Phase 1: Core Functionality (MVP)
**Target: 2-3 days**
- [ ] Project setup (Cargo, dependencies)
- [ ] Config & task models
- [ ] `init`, `new`, `list`, `show` commands
- [ ] Basic file I/O and frontmatter parsing
- [ ] Unit tests

### Phase 2: Full CLI (Feature Complete)
**Target: 2-3 days**
- [ ] `update`, `move`, `open` commands
- [ ] `search` with ripgrep integration
- [ ] `save` with git integration
- [ ] Filtering and sorting options
- [ ] Integration tests

### Phase 3: Polish & Performance
**Target: 1-2 days**
- [ ] Error handling improvements
- [ ] Colored output and formatting
- [ ] Performance benchmarks
- [ ] Documentation (README, examples)
- [ ] CI/CD pipeline

### Phase 4: Distribution
**Target: 1 day**
- [ ] Release builds for multiple platforms
- [ ] Homebrew formula
- [ ] Crates.io publication
- [ ] GitHub releases automation

## Success Metrics

### Performance Targets
- `tasks list`: < 50ms for 1000 tasks
- `tasks show`: < 10ms
- `tasks new`: < 100ms (excluding editor time)
- `tasks search`: < 200ms for 1000 tasks
- `tasks save`: < 150ms
- Binary size: < 5MB (stripped)
- Memory usage: < 10MB for typical operations

### Code Quality
- Test coverage: > 80%
- Zero unsafe code (except in dependencies)
- No clippy warnings on `--all-features`
- All commands documented with `--help`

## Future Enhancements (Post-MVP)

### Short Term
- Shell completion scripts (bash, zsh, fish)
- Dependency visualization (`tasks graph`)
- Task templates
- Interactive task creation wizard
- Bulk operations (`tasks move --tag=bug done`)

### Long Term
- TUI (Terminal UI) mode with `ratatui`
- Watch mode for auto-refresh
- Task time tracking
- Integration with GitHub Issues
- Plugin system for custom commands

## LLM Integration Considerations

The file-based architecture naturally supports LLM manipulation:

1. **Direct File Access**: LLMs can use file tools to read/write tasks
2. **Standard Format**: YAML frontmatter is widely understood
3. **Self-Documenting**: Task structure is evident from examples
4. **No Database**: No need for LLM to interact with complex APIs

**Example LLM Workflow:**
```
Human: "Create a task for implementing search functionality"
LLM: [Reads config.json to understand project structure]
     [Generates timestamp ID: 20260108160000]
     [Writes .repo-tasks/tasks/todo/20260108160000-implement-search.md]
     [Optionally runs: tasks save -m "Created search task"]
```

## Risks & Mitigations

### Risk: Rust Learning Curve
**Mitigation:**
- Well-documented code
- Follow Rust API guidelines
- Use idiomatic patterns from ecosystem

### Risk: Git Integration Complexity
**Mitigation:**
- Use battle-tested `git2-rs` library
- Keep git operations simple (add, commit only)
- Comprehensive error handling

### Risk: Cross-Platform Compatibility
**Mitigation:**
- Use `std::path::Path` (not string manipulation)
- Test on Windows, macOS, Linux in CI
- Use `console` crate for terminal abstraction

## Conclusion

Building `repo-tasks` in Rust with the proposed architecture will deliver:
- **Sub-100ms performance** for all core operations
- **5MB binary** with zero runtime dependencies
- **Robust error handling** with helpful messages
- **Excellent developer experience** with modern CLI patterns
- **LLM-friendly design** through simple file structures

The combination of Rust's performance, excellent CLI ecosystem, and the simple file-based architecture makes this the optimal approach for a fast, maintainable task management tool.
