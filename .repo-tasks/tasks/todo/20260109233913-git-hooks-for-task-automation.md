---
ID: "20260109233913"
Title: Add git hooks for automated task status updates
Priority: High
Tags:
  - git
  - automation
  - workflow
  - hooks
---

## Overview

Implement git hooks that automatically update task statuses based on commit messages and git operations, creating a seamless integration between code commits and task tracking.

## Proposed Git Hooks

### 1. prepare-commit-msg Hook

**Purpose:** Help users reference tasks in commit messages

**Features:**
- Detect current branch name for task ID patterns (e.g., `task/20260109120001`)
- Auto-inject task reference into commit message template
- Show current task context when committing

**Example:**
```bash
# Branch: task/20260109120001-feature-name
# Auto-generates commit template:
[20260109120001]

# Current task: Create cross-platform release builds
# Status: in-progress
```

### 2. post-commit Hook

**Purpose:** Automatically update task status based on commit message keywords

**Keywords to detect:**
- `[done]`, `[complete]`, `[finished]` → Move task to `done`
- `[testing]`, `[review]` → Move task to `testing`
- `[in-progress]`, `[wip]` → Move task to `in-progress`
- `[blocked]`, `[paused]` → Add note to task
- `closes #<task-id>`, `fixes #<task-id>` → Move to done

**Example:**
```bash
git commit -m "[20260109120001] Add cross-platform builds [done]"
# Automatically moves task 20260109120001 to done status
```

### 3. pre-commit Hook

**Purpose:** Validate task references and prevent commits to protected statuses

**Validations:**
- Warn if committing without task reference in branch/message
- Check if referenced task exists
- Prevent commits if task is in `done` status (configurable)
- Validate task ID format

### 4. post-checkout Hook

**Purpose:** Show relevant task information when switching branches

**Features:**
- Detect task ID from branch name
- Display task details when checking out task branches
- Suggest moving task to in-progress if in todo

**Example:**
```bash
git checkout task/20260109120001-feature
# Shows:
# 🔴 Critical [20260109120001] Create cross-platform release builds
# Status: todo
#
# Tip: Run 'repo-tasks move 20260109120001 in-progress' to update status
```

## Implementation Details

### Hook Installation

Add new command: `repo-tasks hooks install`

Options:
- `--global` - Install hooks globally for all repos
- `--local` - Install hooks for current repo only (default)
- `--uninstall` - Remove installed hooks
- `--which <hook-name>` - Install specific hook only

### Hook Configuration

Add to `.repo-tasks/config.json`:

```json
{
  "hooks": {
    "enabled": true,
    "auto_status_update": true,
    "require_task_reference": false,
    "keywords": {
      "done": ["done", "complete", "finished", "closes", "fixes"],
      "testing": ["testing", "review", "ready"],
      "in-progress": ["wip", "in-progress", "started"]
    }
  }
}
```

### Safety Features

1. **Dry-run mode** - Show what would change without making changes
2. **Confirmation prompts** - Ask before auto-updating tasks (configurable)
3. **Hook bypass** - Support `git commit --no-verify` to skip hooks
4. **Logging** - Log all hook actions to `.repo-tasks/hooks.log`
5. **Error handling** - Never block commits if hook fails

## User Experience

### Opt-in by default
Hooks should be opt-in and easy to enable:

```bash
# During init
repo-tasks init
# Prompt: Install git hooks for automated task updates? [y/N]

# Or later
repo-tasks hooks install
```

### Status feedback
Hooks should provide clear feedback:

```bash
git commit -m "[20260109120001] Implement feature [done]"
# ✓ Committed: 78af672
# ✓ Moved task 20260109120001: in-progress → done
```

## Technical Implementation

### Hook Scripts Location

Store hook templates in the binary or as embedded resources:
- `src/hooks/templates/prepare-commit-msg.sh`
- `src/hooks/templates/post-commit.sh`
- `src/hooks/templates/pre-commit.sh`
- `src/hooks/templates/post-checkout.sh`

### Hook Installation Process

1. Check if `.git/hooks/` exists
2. Backup existing hooks (rename to `.backup`)
3. Install repo-tasks hooks
4. Make hooks executable
5. Create hook registry file to track installed hooks

### Parsing Commit Messages

Use regex patterns to extract:
- Task IDs: `[20260109120001]`, `#20260109120001`, `task/20260109120001`
- Keywords: `[done]`, `[testing]`, etc.
- GitHub-style: `closes #20260109120001`, `fixes #20260109120001`

### Calling repo-tasks CLI

Hooks should call the repo-tasks binary:

```bash
# In post-commit hook
TASK_ID=$(echo "$COMMIT_MSG" | grep -oE '[0-9]{14}' | head -1)
if [ -n "$TASK_ID" ]; then
  repo-tasks move "$TASK_ID" done --quiet 2>/dev/null || true
fi
```

## Testing Strategy

1. **Unit tests** - Test commit message parsing logic
2. **Integration tests** - Test hook installation and execution
3. **Manual testing** - Test in real repository with various scenarios
4. **Edge cases**:
   - Multiple task IDs in one commit
   - Invalid task IDs
   - Task already in target status
   - Hooks in subdirectories of git repo

## Documentation

Update docs with:
- How to install hooks
- List of supported keywords
- Configuration options
- How to customize hook behavior
- Troubleshooting guide

## Benefits

1. **Reduced friction** - No need to manually update task status after commits
2. **Better tracking** - Automatic status updates keep task board accurate
3. **Workflow integration** - Git and tasks work together seamlessly
4. **Team collaboration** - Consistent task updates across team members
5. **Audit trail** - Git history and task history stay in sync

## Related Tasks

- #20260109231855 - Non-interactive CLI mode (needed for hooks)
- Future: GitHub Actions integration for PR-based task updates

## Files to Create/Modify

- `src/commands/hooks.rs` - New hooks command
- `src/hooks/` - Hook implementation module
- `src/hooks/templates/` - Hook script templates
- `src/hooks/parser.rs` - Commit message parsing logic
- Update `src/main.rs` to include hooks command
- Update configuration schema for hook settings
