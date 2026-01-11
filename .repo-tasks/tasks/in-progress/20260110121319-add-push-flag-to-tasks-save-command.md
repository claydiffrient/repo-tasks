---
ID: '20260110121319'
Title: Add --push flag to tasks save command
Priority: Medium
Tags:
- cli
- enhancement
- git
---

Add a --push flag to the 'tasks save' command that automatically pushes to the remote repository after committing task files.

## Problem

Currently, after using 'tasks save' to commit task files, users need to manually run 'git push' to push the changes to the remote. This adds friction to the workflow.

## Proposed Solution

Add a '--push' flag to 'tasks save':

```bash
tasks save --push
# Commits task files, then pushes to remote
```

## Implementation Details

In src/commands/save.rs:
1. Add '--push' flag to command arguments
2. After successful commit, check if remote is configured
3. If remote exists and --push is used, run git push
4. Handle errors gracefully (no remote, push fails, etc.)

## Behavior

```bash
# Default behavior (no change)
tasks save
# ✓ Committed changes: Update tasks: 2 modified

# With --push flag
tasks save --push
# ✓ Committed changes: Update tasks: 2 modified
# ✓ Pushed to remote: origin/main

# If no remote configured
tasks save --push
# ✓ Committed changes: Update tasks: 2 modified
# ⚠ Warning: No remote configured, skipping push

# If push fails
tasks save --push
# ✓ Committed changes: Update tasks: 2 modified
# ✗ Push failed: [error details]
# Tip: Run 'git push' manually or check remote configuration
```

## Error Handling

- Detect if remote exists using git2 crate
- If no remote: Show warning, don't fail
- If push fails: Show error but commit was successful
- Clear messaging about what succeeded/failed

## Configuration Option (Future)

Could add config option to make push default:

```json
{
  "auto_push": true  // Default to pushing after save
}
```

## Benefits

- Reduces workflow friction
- Fewer commands to remember
- Consistent with 'git commit && git push' pattern
- Optional (doesn't change default behavior)

## Testing

- Test with remote configured
- Test without remote
- Test with push failure (auth, conflicts, etc.)
- Test with multiple remotes