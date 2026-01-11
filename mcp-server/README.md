# repo-tasks MCP Server

Model Context Protocol (MCP) server for [repo-tasks](../README.md) task management. This server allows LLMs like Claude to interact with your tasks through natural language.

## Features

The MCP server exposes all repo-tasks functionality through MCP tools:

### Task Management
- **list_tasks** - List tasks by status, priority, or tag
- **show_task** - Display detailed task information
- **create_task** - Create new tasks with title, priority, tags, and notes
- **update_task** - Update existing task properties
- **move_task** - Move tasks between statuses
- **start_task** - Begin work on a task (move to in-progress + create git branch)

### Search & Discovery
- **search_tasks** - Full-text search across all tasks with regex support

### Version Control
- **save_tasks** - Commit task changes to git

### Editor Integration
- **open_task** - Open task in default editor

## Installation

### Prerequisites

1. **repo-tasks CLI** must be installed:
   ```bash
   cargo install --path ..
   # or if already installed
   cargo install repo-tasks
   ```

2. **Node.js** (v18 or later) for running the MCP server

### Setup

1. Install dependencies:
   ```bash
   cd mcp-server
   npm install
   ```

2. Build the TypeScript code:
   ```bash
   npm run build
   ```

3. Test the server:
   ```bash
   node dist/index.js
   ```

## Configuration

### Claude Code

Add to your Claude Code MCP settings (`~/.config/claude-code/mcp_settings.json`):

```json
{
  "mcpServers": {
    "repo-tasks": {
      "command": "node",
      "args": [
        "/absolute/path/to/repo-tasks/mcp-server/dist/index.js"
      ]
    }
  }
}
```

Replace `/absolute/path/to/repo-tasks` with the actual path to your repo-tasks repository.

After adding the configuration, restart Claude Code or reload the MCP servers.

### Claude Desktop

The server also works with Claude Desktop. Add to `~/Library/Application Support/Claude/claude_desktop_config.json` on macOS:

```json
{
  "mcpServers": {
    "repo-tasks": {
      "command": "node",
      "args": [
        "/absolute/path/to/repo-tasks/mcp-server/dist/index.js"
      ]
    }
  }
}
```

### Other MCP Clients

The server uses stdio transport and follows the standard MCP protocol. Configure your MCP client to run:

```bash
node /path/to/mcp-server/dist/index.js
```

## Usage Examples

Once configured, you can interact with repo-tasks through natural language in Claude Code:

### Creating Tasks
```
Create a new high priority task titled "Add user authentication" with tags "security,backend"
```

### Listing and Searching
```
Show me all tasks in the todo status

Search for tasks related to "authentication"

List all high priority tasks
```

### Working on Tasks
```
Start working on task 20260110184347

Move task create-start-command to testing

Show details for task 20260110201217
```

### Version Control
```
Save all task changes with message "Update sprint tasks"
```

## Tool Reference

### list_tasks

List tasks with optional filters.

**Parameters:**
- `status` (string, optional): Filter by status (todo, in-progress, testing, done). Default: "todo"
- `priority` (string, optional): Filter by priority (Low, Medium, High, Critical)
- `tag` (string, optional): Filter by tag

**Example:**
```typescript
{
  "status": "in-progress",
  "priority": "High"
}
```

### show_task

Show detailed information about a specific task.

**Parameters:**
- `task_id` (string, required): Task ID or slug

**Example:**
```typescript
{
  "task_id": "20260110184347"
}
```

### create_task

Create a new task.

**Parameters:**
- `title` (string, required): Task title
- `priority` (string, optional): Priority level. Default: "Medium"
- `tags` (string, optional): Comma-separated tags
- `notes` (string, optional): Task description/notes

**Example:**
```typescript
{
  "title": "Implement user authentication",
  "priority": "High",
  "tags": "security,backend",
  "notes": "Add JWT-based authentication system"
}
```

### update_task

Update task properties (interactive).

**Parameters:**
- `task_id` (string, required): Task ID or slug

### move_task

Move a task to a different status.

**Parameters:**
- `task_id` (string, required): Task ID or slug
- `new_status` (string, required): New status (todo, in-progress, testing, done)

**Example:**
```typescript
{
  "task_id": "20260110184347",
  "new_status": "testing"
}
```

### start_task

Start working on a task (recommended for beginning work).

Moves the task to in-progress and creates a git branch with format `{id}-{slug}`.

**Parameters:**
- `task_id` (string, required): Task ID or slug

**Example:**
```typescript
{
  "task_id": "20260110184347"
}
```

### search_tasks

Search tasks by content.

**Parameters:**
- `query` (string, required): Search query (regex supported)

**Example:**
```typescript
{
  "query": "authentication"
}
```

### save_tasks

Commit task changes to git.

**Parameters:**
- `message` (string, optional): Commit message (auto-generated if not provided)

**Example:**
```typescript
{
  "message": "Update sprint tasks"
}
```

### open_task

Open a task in the default editor.

**Parameters:**
- `task_id` (string, required): Task ID or slug

## Development

### Watch Mode

Run TypeScript compiler in watch mode:

```bash
npm run watch
```

### Testing

Test the server manually using the MCP inspector:

```bash
npx @modelcontextprotocol/inspector node dist/index.js
```

## Troubleshooting

### "repo-tasks binary not found"

The server looks for the repo-tasks binary in:
1. System PATH
2. `./target/release/repo-tasks` (local release build)
3. `./target/debug/repo-tasks` (local debug build)

Ensure repo-tasks is installed or built from source.

### Changes not reflected

After modifying the TypeScript code, rebuild:

```bash
npm run build
```

Then restart Claude Code to reload the MCP server.

### Server not showing in Claude

1. Check the configuration file path (`~/.config/claude-code/mcp_settings.json`)
2. Verify the absolute path to `dist/index.js` is correct
3. Check Claude Code logs for errors
4. Restart Claude Code after configuration changes

## License

Same as repo-tasks: MIT OR Apache-2.0
