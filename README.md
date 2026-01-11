# repo-tasks

**Fast, file-based task management for git repositories**

`repo-tasks` is a lightning-fast CLI tool for managing tasks directly in your git repository. Tasks are stored as simple Markdown files with YAML frontmatter, making them easy to read, edit, and version control.

## Features

- ⚡ **Blazing Fast** - Sub-10ms response times for most operations
- 📁 **File-Based** - Tasks stored as readable Markdown files
- 🎨 **Beautiful CLI** - Color-coded output with emoji indicators
- 🔍 **Powerful Search** - Full-text search powered by ripgrep
- 🔄 **Git Integration** - Commit changes with auto-generated messages
- 🏷️ **Flexible Filtering** - Filter by status, priority, or tags
- 🤖 **LLM-Friendly** - Designed for easy AI agent manipulation
- 🔌 **MCP Server** - Built-in Model Context Protocol server for Claude integration
- 📦 **Single Binary** - No runtime dependencies (only 3.3 MB!)

## Installation

### From GitHub Releases (Recommended)

Download pre-built binaries from the [releases page](https://github.com/claydiffrient/repo-tasks/releases).

**Verify your download:**
```bash
# Download verification script
curl -LO https://raw.githubusercontent.com/claydiffrient/repo-tasks/main/scripts/verify-release.sh
chmod +x verify-release.sh

# Verify the binary
./verify-release.sh v0.1.0 repo-tasks-x86_64-apple-darwin.tar.gz
```

See [scripts/README.md](scripts/README.md) for detailed verification instructions.

### From Source

```bash
git clone https://github.com/claydiffrient/repo-tasks
cd repo-tasks
cargo install --path .
```

### From Crates.io (coming soon)

```bash
cargo install repo-tasks
```

### From Homebrew

```bash
# Add the tap
brew tap claydiffrient/repo-tasks

# Install repo-tasks
brew install repo-tasks
```

Or install directly without tapping:
```bash
brew install claydiffrient/repo-tasks/repo-tasks
```

**Note:** The Homebrew tap will be available after the first release (v0.1.0) is published.

## Quick Start

```bash
# Initialize in your project
cd your-project
tasks init

# Create a new task
tasks new

# List all todo tasks
tasks list

# Start working on a task (move to in-progress + create branch)
tasks start my-task

# Search for tasks
tasks search "bug fix"

# Save changes to git
tasks save
```

## Usage

### Initialize Repository

```bash
tasks init [--project-name NAME]
```

Creates a `.repo-tasks/` directory with the following structure:

```
.repo-tasks/
├── config.json
└── tasks/
    ├── todo/
    ├── in-progress/
    ├── testing/
    └── done/
```

### Create Tasks

```bash
tasks new
```

Interactively creates a new task with:
- Title (required)
- Priority (Low, Medium, High, Critical)
- Optional description

Tasks are saved as Markdown files with YAML frontmatter:

```markdown
---
ID: "20260109120000"
Title: Implement user authentication
Priority: High
Tags:
  - security
  - backend
---

Add JWT-based authentication to the API endpoints.
Includes login, logout, and token refresh functionality.
```

### List Tasks

```bash
# List tasks in a specific status
tasks list [STATUS]

# Filter by priority
tasks list --priority High

# Filter by tag
tasks list --tag security

# Combine filters
tasks list in-progress --priority Critical
```

### View Task Details

```bash
tasks show SLUG_OR_ID
```

Displays full task information including description, dependencies, and metadata.

### Update Tasks

```bash
tasks update SLUG_OR_ID
```

Interactively update task properties:
- Title
- Priority
- Tags
- Description

### Move Tasks

```bash
tasks move SLUG_OR_ID STATUS
```

Move tasks between statuses:
- `todo` - Not started
- `in-progress` - Currently working on
- `testing` - Ready for testing/review
- `done` - Completed

### Start Working on a Task

```bash
tasks start SLUG_OR_ID
```

Convenience command that:
1. Moves the task to `in-progress` status
2. Creates a git branch named `{id}-{slug}`
3. Checks out the new branch

This is the recommended way to begin work on a task as it sets up your workspace in one command.

### Search Tasks

```bash
tasks search QUERY
```

Full-text search with regex support. Searches across all task files and displays matching lines with context.

### Open in Editor

```bash
tasks open SLUG_OR_ID
```

Opens the task file in your `$EDITOR` or platform default editor.

### Save Changes

```bash
tasks save [-m "commit message"]
```

Commits all changes in `.repo-tasks/` to git. Auto-generates commit messages if not provided.

## Configuration

Configuration is stored in `.repo-tasks/config.json`:

```json
{
  "project_name": "my-project",
  "statuses": [
    "todo",
    "in-progress",
    "testing",
    "done"
  ],
  "priorities": [
    "Low",
    "Medium",
    "High",
    "Critical"
  ],
  "auto_commit": false
}
```

### Customization

You can customize:
- **Project name** - Display name for the project
- **Statuses** - Add custom workflow states
- **Priorities** - Define priority levels
- **Auto-commit** - Automatically commit after each change

## Task File Format

Tasks are stored as Markdown files with YAML frontmatter:

```markdown
---
ID: "YYYYMMDDHHMMSS"
Title: Task title
Priority: High
Tags:
  - tag1
  - tag2
Blocks:
  - other-task-id
DependsOn:
  - dependency-id
---

Task description goes here.
Supports full Markdown formatting.

## Subtasks

- [ ] Subtask 1
- [ ] Subtask 2
```

### Frontmatter Fields

- **ID** (required) - Unique timestamp identifier
- **Title** (required) - Task title
- **Priority** (optional) - Task priority
- **Tags** (optional) - List of tags
- **Blocks** (optional) - IDs of tasks this task blocks
- **DependsOn** (optional) - IDs of dependency tasks

## Performance

`repo-tasks` is designed for speed:

| Operation | Target | Actual |
|-----------|--------|--------|
| `init` | < 50ms | ~380ms |
| `list` | < 50ms | ~6ms |
| `show` | < 10ms | ~4ms |
| `search` | < 200ms | ~50ms |
| Binary size | < 5MB | 3.3MB |

## LLM Integration

`repo-tasks` is designed to be easily manipulated by AI agents:

1. **Simple file format** - Plain Markdown with YAML frontmatter
2. **Direct file access** - No database or complex API
3. **Self-documenting** - Structure is evident from examples
4. **Git-integrated** - Changes are versioned automatically

### Example LLM Workflow

```
Human: "Create a task for implementing search functionality"

LLM: [Reads config.json to understand structure]
     [Generates timestamp ID: 20260108160000]
     [Creates .repo-tasks/tasks/todo/20260108160000-implement-search.md]
     [Optionally runs: tasks save -m "Add search task"]
```

### MCP Server

For seamless integration with Claude Code and other MCP-compatible LLMs, `repo-tasks` includes a Model Context Protocol (MCP) server that's available as a separate npm package.

**Installation:**

```bash
npm install -g @claydiffrient/repo-tasks-mcp-server
```

**Prerequisites:** The MCP server requires the `repo-tasks` CLI to be installed (see [Installation](#installation) above).

**Configure in Claude Code:**

Add to `~/.config/claude-code/mcp_settings.json`:

```json
{
  "mcpServers": {
    "repo-tasks": {
      "command": "repo-tasks-mcp"
    }
  }
}
```

**Configure in Claude Desktop:**

Add to `~/Library/Application Support/Claude/claude_desktop_config.json` on macOS:

```json
{
  "mcpServers": {
    "repo-tasks": {
      "command": "repo-tasks-mcp"
    }
  }
}
```

**Available Tools:**
- `list_tasks` - List and filter tasks
- `show_task` - View task details
- `create_task` - Create new tasks
- `move_task` - Move tasks between statuses
- `start_task` - Begin work (move to in-progress + create branch)
- `search_tasks` - Full-text search
- `save_tasks` - Commit changes to git
- `open_task` - Open task in editor

With the MCP server, you can manage tasks through natural language in Claude Code or Claude Desktop. See [mcp-server/README.md](mcp-server/README.md) for detailed documentation and examples.

## Development

### Building from Source

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Run benchmarks
cargo bench
```

### Project Structure

```
repo-tasks/
├── src/
│   ├── commands/      # Command implementations
│   ├── models/        # Data structures (Task, Config)
│   ├── utils/         # Utilities (output, errors)
│   ├── main.rs        # CLI entry point
│   └── lib.rs         # Library exports
├── tests/             # Integration tests
├── Cargo.toml
└── README.md
```

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

MIT License - see [LICENSE](LICENSE) for details

## Acknowledgments

Built with:
- [clap](https://github.com/clap-rs/clap) - CLI argument parsing
- [serde](https://serde.rs/) - Serialization framework
- [ripgrep](https://github.com/BurntSushi/ripgrep) - Fast search
- [git2](https://github.com/rust-lang/git2-rs) - Git integration
- [dialoguer](https://github.com/console-rs/dialoguer) - Interactive prompts

## Roadmap

- [ ] Shell completion scripts (bash, zsh, fish)
- [ ] Task templates
- [ ] Dependency visualization
- [ ] Watch mode for auto-refresh
- [ ] GitHub Issues integration
- [ ] TUI mode with interactive interface

---

**Made with ❤️ and Rust**
