# Changelog

All notable changes to the repo-tasks MCP server will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2026-01-11

### Fixed

- Binary detection now checks for both `tasks` (Homebrew default) and `repo-tasks` names
- Updated error message to mention both binary names
- Prioritizes `tasks` name check first for better Homebrew compatibility

## [0.1.0] - 2026-01-11

### Added

- Initial release of the MCP server for repo-tasks
- Published to npm as `@claydiffrient/repo-tasks-mcp-server`
- Core task management tools:
  - `list_tasks` - List and filter tasks by status, priority, or tag
  - `show_task` - Display detailed task information
  - `create_task` - Create new tasks with title, priority, tags, and notes
  - `update_task` - Update existing task properties (interactive)
  - `move_task` - Move tasks between statuses
  - `start_task` - Begin work on a task (move to in-progress + create git branch)
- Search functionality:
  - `search_tasks` - Full-text search across all tasks with regex support
- Version control integration:
  - `save_tasks` - Commit task changes to git with auto-generated or custom messages
- Editor integration:
  - `open_task` - Open task file in default editor
- Post-install check that verifies repo-tasks CLI is installed
- Comprehensive README with installation and configuration instructions
- Support for both Claude Code and Claude Desktop

### Dependencies

- `@modelcontextprotocol/sdk` ^1.0.0 - Model Context Protocol implementation
- Requires `repo-tasks` CLI to be installed separately

### Known Limitations

- Requires `repo-tasks` CLI to be installed on the system
- Interactive commands (like `update_task`) may not work well in all MCP clients
- Currently only supports stdio transport (no HTTP/SSE transport)
- No built-in task templates or advanced filtering beyond basic status/priority/tag

### Installation

```bash
npm install -g @claydiffrient/repo-tasks-mcp-server
```

### Configuration

**Claude Code** (`~/.config/claude-code/mcp_settings.json`):
```json
{
  "mcpServers": {
    "repo-tasks": {
      "command": "repo-tasks-mcp"
    }
  }
}
```

**Claude Desktop** (`~/Library/Application Support/Claude/claude_desktop_config.json`):
```json
{
  "mcpServers": {
    "repo-tasks": {
      "command": "repo-tasks-mcp"
    }
  }
}
```

[0.1.0]: https://github.com/claydiffrient/repo-tasks/releases/tag/v0.1.0
