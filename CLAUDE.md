# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Overview

`repo-tasks` is a fast, file-based task management CLI tool for git repositories written in Rust. Tasks are stored as Markdown files with YAML frontmatter in a `.repo-tasks/` directory, making them easy to read, edit, and version control.

**Key Design Principles:**
- Performance-first: Sub-100ms response times for most operations
- Simple data model: No database, just files with YAML frontmatter
- LLM-friendly: Tasks can be manipulated directly via file operations
- Git-integrated: Native git operations for version control

## Common Commands

### Development Workflow

```bash
# Build and run (debug)
cargo run -- <command>

# Build optimized release version
cargo build --release

# Run tests
cargo test

# Run specific test
cargo test test_name

# Run benchmarks
cargo bench

# Run clippy linter
cargo clippy --all-features

# Format code
cargo fmt
```

### Testing the CLI

```bash
# Test commands locally (debug build)
cargo run -- init
cargo run -- new
cargo run -- list
cargo run -- show <slug-or-id>
cargo run -- move <slug-or-id> in-progress
cargo run -- search "query"
cargo run -- save

# Test with release build
cargo run --release -- list
```

### Installation

```bash
# Install locally
cargo install --path .

# After installation, use directly
tasks init
tasks new
```

## Architecture

### Directory Structure

```
src/
├── main.rs           # CLI entry point with clap command definitions
├── lib.rs            # Library exports (Config, Task)
├── commands/         # Command implementations (one file per command)
│   ├── mod.rs
│   ├── init.rs       # Initialize .repo-tasks/
│   ├── new.rs        # Create new task
│   ├── list.rs       # List tasks with filters
│   ├── show.rs       # Display task details
│   ├── update.rs     # Update task properties
│   ├── move_task.rs  # Move task between statuses
│   ├── open.rs       # Open task in editor
│   ├── search.rs     # Full-text search with ripgrep
│   └── save.rs       # Git commit operations
├── models/           # Core data structures
│   ├── mod.rs
│   ├── config.rs     # Config struct (loaded from config.json)
│   └── task.rs       # Task struct with frontmatter parsing
└── utils/            # Shared utilities
    ├── mod.rs
    ├── output.rs     # Terminal output formatting
    └── errors.rs     # Error handling utilities
```

### Data Model

**Task Structure (`src/models/task.rs`):**
- Tasks serialize to/from Markdown files with YAML frontmatter
- Frontmatter fields: `ID`, `Title`, `Priority`, `Tags`, `Blocks`, `DependsOn`
- The `slug` and `status` are derived from the filename and directory path
- Files named: `{timestamp-id}-{slug}.md` (e.g., `20260109120000-implement-search.md`)

**Config Structure (`src/models/config.rs`):**
- Stored at `.repo-tasks/config.json`
- Contains: `project_name`, `statuses` array, `priorities` array, `auto_commit` flag
- Default statuses: `todo`, `in-progress`, `testing`, `done`
- Default priorities: `Low`, `Medium`, `High`, `Critical`

**File Organization:**
```
.repo-tasks/
├── config.json
└── tasks/
    ├── todo/
    ├── in-progress/
    ├── testing/
    └── done/
```

### Key Implementation Details

**Task ID Generation:**
- Uses timestamp format: `YYYYMMDDHHMMSS` (14 digits)
- Generated via `chrono::Local::now().format("%Y%m%d%H%M%S")`

**Slug Generation:**
- Uses the `slug` crate to convert titles to URL-safe slugs
- Example: "Create Requirements Document" → "create-requirements-document"

**Frontmatter Parsing:**
- Split content by `---` delimiters (YAML frontmatter format)
- Parse YAML using `serde_yaml`
- Body content follows the closing `---`

**Search Implementation:**
- Uses `grep-rs` (ripgrep library) for fast full-text search
- Multi-threaded search across all task files
- Supports regex patterns

**Git Integration:**
- Uses `git2` crate (libgit2 bindings) for native git operations
- Avoids shelling out to `git` command for better performance
- Stages `.repo-tasks/` directory and creates commits

### Performance Considerations

**Critical Performance Targets:**
- `init`: < 50ms
- `list`: < 50ms for 1000 tasks
- `show`: < 10ms
- `search`: < 200ms for 1000 tasks
- Binary size: < 5MB (currently 3.3MB)

**Optimization Techniques:**
1. **Lazy loading**: `list` command only parses frontmatter, not full body
2. **No database**: Direct file I/O is fastest for this scale
3. **Release optimizations**: LTO, single codegen unit, symbol stripping (see `Cargo.toml` profile)
4. **Minimal allocations**: Use string slices where possible
5. **Parallel search**: ripgrep's multi-threaded search for full-text queries

### Error Handling

- Uses `anyhow` for application errors with context
- Provides user-friendly error messages with `.context()`
- All functions return `anyhow::Result<T>` for consistent error propagation

## LLM Integration Notes

This tool is designed to be easily manipulated by AI agents:

1. **Direct File Access**: AI can read/write task files directly without using the CLI
2. **Standard Format**: YAML frontmatter is widely understood by LLMs
3. **Self-Documenting**: Structure is evident from examining example files
4. **No Complex API**: Simple file operations are sufficient

**Example LLM workflow:**
```
1. Read .repo-tasks/config.json to understand structure
2. Generate timestamp ID: chrono::Local::now().format("%Y%m%d%H%M%S")
3. Generate slug from title using slugify()
4. Write .repo-tasks/tasks/todo/{id}-{slug}.md with frontmatter
5. Optionally: Run `tasks save -m "Add new task"`
```

## Testing

**Unit Tests:**
- Located in the same file as the code (inline `#[cfg(test)]` modules)
- Run with `cargo test`

**Integration Tests:**
- Located in `tests/integration_tests.rs`
- Use `assert_cmd` and `predicates` crates for CLI testing
- Use `tempfile` for temporary test directories

**Benchmarks:**
- Located in `benches/task_operations.rs`
- Use `criterion` crate for statistical benchmarking
- Run with `cargo bench`

## Dependencies

**Core:**
- `clap` - CLI argument parsing with derive API
- `serde` + `serde_yaml` + `serde_json` - Serialization
- `anyhow` - Error handling with context
- `chrono` - Timestamp generation
- `slug` - Slug generation from titles

**File Operations:**
- `walkdir` - Directory traversal
- Standard library `std::fs` for direct I/O

**Search:**
- `grep-searcher`, `grep-matcher`, `grep-regex` - ripgrep library

**Git:**
- `git2` - libgit2 bindings for native git operations

**Terminal UI:**
- `dialoguer` - Interactive prompts
- `console` - Terminal styling
- `atty` - TTY detection
