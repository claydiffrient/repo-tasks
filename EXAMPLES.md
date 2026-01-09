# Examples

This document provides practical examples of using `repo-tasks` in real-world scenarios.

## Basic Workflow

### Setting Up a New Project

```bash
# Navigate to your project
cd my-project

# Initialize repo-tasks
tasks init --project-name "My Project"

# Create your first task
tasks new
# Enter: "Set up project structure"
# Select priority: High

# View your tasks
tasks list
```

### Daily Development Workflow

```bash
# Start your day - see what's pending
tasks list todo

# Pick a task and start working on it
tasks move setup-project-structure in-progress

# Check details while working
tasks show setup-project-structure

# Mark it as ready for testing
tasks move setup-project-structure testing

# After testing passes
tasks move setup-project-structure done

# Commit your progress
tasks save -m "Complete project setup"
```

## Advanced Usage

### Managing a Feature Branch

```bash
# Create tasks for a new feature
tasks new
# Title: "Design user profile API"
# Priority: High
# Tags: api, backend

tasks new
# Title: "Implement profile endpoints"
# Priority: High
# Tags: api, backend

tasks new
# Title: "Add profile tests"
# Priority: Medium
# Tags: api, testing

# View all API-related tasks
tasks list --tag api

# Track progress
tasks list in-progress --tag backend
```

### Bug Triage

```bash
# Create bug tasks
for i in {1..5}; do
  tasks new
  # Title: "Fix bug #$i"
  # Priority: varies
  # Tags: bug, urgent
done

# See all bugs
tasks list --tag bug

# Prioritize critical bugs
tasks list --priority Critical --tag bug

# Move urgent bug to in-progress
tasks move fix-bug-1 in-progress
```

### Sprint Planning

```bash
# Create sprint tasks
tasks new  # "User authentication"
tasks new  # "Dashboard redesign"
tasks new  # "Performance optimization"
tasks new  # "Documentation update"

# View sprint backlog
tasks list todo

# Filter by priority for sprint selection
tasks list todo --priority High
tasks list todo --priority Medium

# Move selected tasks to in-progress
tasks move user-authentication in-progress
tasks move dashboard-redesign in-progress

# Track sprint progress
tasks list in-progress
tasks list testing
tasks list done
```

## Filtering and Search

### Find Tasks by Content

```bash
# Search for authentication-related tasks
tasks search "auth"

# Search for error handling
tasks search "error|exception"

# Search for TODO comments in task descriptions
tasks search "TODO:"
```

### Complex Filtering

```bash
# High priority tasks not yet started
tasks list todo --priority High

# All tasks with security tag
for status in todo in-progress testing done; do
  echo "=== $status ==="
  tasks list $status --tag security
done

# Critical bugs in progress
tasks list in-progress --priority Critical
```

## Task Updates

### Bulk Update Tags

```bash
# Add tags to existing tasks
tasks update api-endpoint-1
# Select: Tags
# Enter: api, v2, public

tasks update api-endpoint-2
# Select: Tags
# Enter: api, v2, internal
```

### Changing Priorities

```bash
# Bug becomes critical
tasks update security-vulnerability
# Select: Priority
# Choose: Critical

# Feature can wait
tasks update nice-to-have-feature
# Select: Priority
# Choose: Low
```

## Git Integration

### Automatic Commits

```bash
# Make several changes
tasks new  # Create task 1
tasks new  # Create task 2
tasks move some-task done

# Save all at once with auto-generated message
tasks save
# Output: "Committed: Update tasks: 2 added, 1 modified"
```

### Manual Commit Messages

```bash
# Save with descriptive message
tasks save -m "Add sprint 3 tasks and complete API endpoints"

# Save after major milestone
tasks save -m "Complete authentication feature

All authentication tasks moved to done:
- User login
- Password reset
- JWT tokens
- Refresh tokens"
```

## Editor Integration

### Quick Task Editing

```bash
# Open task in editor
tasks open feature-implementation

# Make changes, save, and close
# Changes are automatically reflected

# Verify changes
tasks show feature-implementation
```

### Batch Editing

```bash
# Open multiple tasks for editing
for task in task-1 task-2 task-3; do
  tasks open $task
done
```

## Productivity Patterns

### Morning Standup

```bash
#!/bin/bash
echo "=== In Progress ===="
tasks list in-progress

echo -e "\n=== Blockers ==="
# Assuming you tag blocked tasks
tasks search "blocked"

echo -e "\n=== Today's Plan ==="
tasks list todo --priority High | head -n 3
```

### End of Day Review

```bash
#!/bin/bash
echo "=== Completed Today ==="
# Show done tasks (you'd normally filter by date)
tasks list done | tail -n 10

echo -e "\n=== Still In Progress ==="
tasks list in-progress

echo -e "\n=== Tomorrow's Priorities ==="
tasks list todo --priority Critical
tasks list todo --priority High

# Save the day's work
tasks save -m "End of day: $(date +%Y-%m-%d)"
```

### Weekly Planning

```bash
#!/bin/bash
echo "=== Week Overview ==="
for status in todo in-progress testing done; do
  count=$(tasks list $status 2>/dev/null | grep -c "^\[")
  echo "$status: $count tasks"
done

echo -e "\n=== Next Week's Focus ==="
tasks list todo --priority Critical
tasks list todo --priority High
```

## Integration with Other Tools

### GitHub Actions

```yaml
# .github/workflows/tasks.yml
name: Task Check

on: [push, pull_request]

jobs:
  check-tasks:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Install repo-tasks
        run: cargo install --path .
      - name: List open tasks
        run: tasks list todo
      - name: Check for critical tasks
        run: |
          if tasks list --priority Critical | grep -q "Critical"; then
            echo "⚠️ Critical tasks found!"
          fi
```

### Pre-commit Hook

```bash
# .git/hooks/pre-commit
#!/bin/bash

# Check if there are untracked task changes
if git diff --name-only | grep -q ".repo-tasks/"; then
  echo "⚠️  Task changes detected!"
  echo ""
  echo "Modified tasks:"
  git diff --name-only | grep ".repo-tasks/" | sed 's/^/  - /'
  echo ""
  echo "Consider running: tasks save"
  echo ""
  read -p "Continue with commit? (y/n) " -n 1 -r
  echo
  if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    exit 1
  fi
fi
```

### Slack Integration

```bash
#!/bin/bash
# notify-slack.sh

STATUS=$1
WEBHOOK_URL="your-webhook-url"

TASKS=$(tasks list $STATUS)
COUNT=$(echo "$TASKS" | grep -c "^\[")

curl -X POST $WEBHOOK_URL \
  -H 'Content-Type: application/json' \
  -d '{
    "text": "📋 *Task Update*\n'"$COUNT"' tasks in *'"$STATUS"'*\n```\n'"$TASKS"'\n```"
  }'
```

## Tips and Tricks

### Alias for Quick Access

```bash
# Add to ~/.bashrc or ~/.zshrc
alias t='tasks'
alias tl='tasks list'
alias tn='tasks new'
alias tm='tasks move'
alias ts='tasks show'
alias tsave='tasks save'
```

### Custom Task IDs

Task IDs are timestamps, but you can create helper scripts:

```bash
#!/bin/bash
# create-task-with-id.sh

TITLE="$1"
ID=$(date +%Y%m%d%H%M%S)
SLUG=$(echo "$TITLE" | tr '[:upper:]' '[:lower:]' | tr ' ' '-')

cat > .repo-tasks/tasks/todo/${ID}-${SLUG}.md <<EOF
---
ID: "$ID"
Title: $TITLE
Priority: Medium
---

Task created via script.
EOF

echo "Created: $SLUG"
```

### Task Dependencies

```bash
# Create dependent tasks
tasks new  # "Set up database" (ID: 123)
tasks new  # "Create API endpoints"

# Edit the second task to depend on the first
tasks open create-api-endpoints
# Add to frontmatter:
# DependsOn:
#   - "123"
```

## Troubleshooting

### Task Not Found

```bash
# If slug is ambiguous, use the full ID
tasks show 20260109120000

# Or search for it
tasks search "partial-name"
```

### Corrupted Task File

```bash
# View the raw file
cat .repo-tasks/tasks/todo/123-broken-task.md

# Fix manually or recreate
tasks open broken-task
# Or delete and recreate
rm .repo-tasks/tasks/todo/123-broken-task.md
tasks new
```

### Git Conflicts

```bash
# If tasks conflict during merge
git status | grep ".repo-tasks/"

# Resolve manually - tasks are just text files
git mergetool .repo-tasks/tasks/todo/conflicted-task.md

# Verify after resolution
tasks show conflicted-task
```
