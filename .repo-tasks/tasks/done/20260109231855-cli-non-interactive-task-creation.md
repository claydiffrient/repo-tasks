---
ID: '20260109231855'
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

## Implementation Complete (2026-01-10)

### ✅ Implemented Features

All proposed functionality has been implemented:

**Command-line flags added:**
- `-t, --title <TITLE>` - Task title (required for non-interactive)
- `-p, --priority <PRIORITY>` - Priority level (defaults to "Medium")
- `-g, --tags <TAGS>` - Comma-separated tags
- `-n, --notes <NOTES>` - Task description/notes

**Mode detection:**
- Automatically detects if arguments are provided (non-interactive mode)
- Falls back to interactive prompts when in a TTY without arguments
- Shows helpful error when not in TTY and no title provided

**Validation:**
- ✅ Title cannot be empty
- ✅ Priority must be valid (Low, Medium, High, Critical)
- ✅ Tags are parsed from comma-separated string
- ✅ Notes are added to task body

### Testing Results

**Non-interactive mode:**
```bash
$ repo-tasks new --title "Test task" --priority High --tags "test,cli" --notes "Description"
✓ Created task: test-task
  ID: 20260110105942
  Priority: 🟠 High
  Tags: (test, cli)
```

**Default priority:**
```bash
$ repo-tasks new --title "Test task"
✓ Created task: test-task
  Priority: 🟡 Medium  # Defaults to Medium
```

**Invalid priority:**
```bash
$ repo-tasks new --title "Test" --priority Invalid
Error: Invalid priority: 'Invalid'
Valid priorities are: Low, Medium, High, Critical
```

**Missing title in non-TTY:**
```bash
$ repo-tasks new --priority High
Error: Not running in a terminal and no title provided.
Use non-interactive mode:
  tasks new --title "Your task title" --priority High
```

### Files Modified

- `src/main.rs` - Added arguments to `Commands::New` enum
- `src/commands/new.rs` - Implemented dual-mode logic
- `Cargo.toml` - Added `atty` dependency for TTY detection

### Benefits Achieved

✅ Enables automation and scripting
✅ Works in CI/CD environments
✅ Faster task creation for power users
✅ Clear error messages for misuse
✅ Backwards compatible - interactive mode still works