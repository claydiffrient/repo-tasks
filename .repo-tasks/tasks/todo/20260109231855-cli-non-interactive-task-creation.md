---
ID: "20260109231855"
Title: Add non-interactive mode for task creation
Priority: High
Tags:
  - cli
  - usability
  - enhancement
  - scripting
---

## Problem

The `repo-tasks new` command only works in interactive mode and requires a TTY. This makes it impossible to:

1. Create tasks from scripts or automation
2. Use the CLI in CI/CD environments
3. Programmatically create tasks
4. Use the tool from non-interactive contexts (like Claude Code)

### Attempted Usage

```bash
# Tried this (wrong command name):
repo-tasks add "Task title" --priority critical --tags ci,bugs

# Tried this (no positional args supported):
repo-tasks new "Task title" --priority critical --tags ci,bugs

# Tried this (requires TTY):
repo-tasks new
# Error: IO error: not a terminal
```

### Current Workaround

Had to manually create markdown files in `.repo-tasks/tasks/todo/` directory with proper frontmatter format.

## Proposed Solution

Add command-line flags to `repo-tasks new` to support non-interactive creation:

```bash
repo-tasks new \
  --title "Fix CI failures" \
  --priority critical \
  --tags ci,github-actions,bugs \
  --notes "Detailed description here..."
```

### Implementation Details

1. **Make flags optional** - If flags are provided, skip interactive prompts
2. **Fall back to interactive** - If no flags provided and TTY available, use interactive mode
3. **Error handling** - If no flags and no TTY, show helpful error with examples
4. **Support stdin** - Consider allowing notes to be piped in: `echo "notes" | repo-tasks new --title "..."`

### Flags to Add

- `--title <TITLE>` or `-t <TITLE>` - Task title (required in non-interactive)
- `--priority <PRIORITY>` or `-p <PRIORITY>` - Priority level (default: medium)
- `--tags <TAGS>` - Comma-separated tags
- `--notes <NOTES>` or `-n <NOTES>` - Task description/notes
- `--status <STATUS>` - Initial status (default: todo)

### Benefits

- Enables automation and scripting
- Better CI/CD integration
- Faster task creation for power users
- Makes tool more versatile and professional

## Files to Modify

- `src/commands/new.rs` - Add argument parsing and non-interactive mode
- Update help text and documentation

## Testing

- Test interactive mode still works when no args provided
- Test non-interactive mode with all flags
- Test error handling when no TTY and no flags
- Test validation of priority/status values in non-interactive mode
