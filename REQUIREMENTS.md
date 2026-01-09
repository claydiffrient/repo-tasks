These are the requirements for the `repo-tasks` project.

# Overview

`repo-tasks` is an application to help manage tasks for small projects, being version controlled
by git and handled with a small script to manage the details.

# Data Structure

## Directory Structure

- `.repo-tasks/` - Root directory containing all data for the application inside the project.
- `.repo-tasks/config.json` - Configuration file for custom settings and repository specifics. The presence of this file indicates the project has been set up to work with `repo-tasks`.
- `.repo-tasks/tasks/[STATUS]/` - Folders named after various [STATUS] states a task might be in: `todo`, `in-progress`, `testing`, `done`.

## Configuration File Schema

`.repo-tasks/config.json` contains project-specific configuration:

```json
{
  "projectName": "repo-tasks",
  "statuses": ["todo", "in-progress", "testing", "done"],
  "priorities": ["Critical", "High", "Medium", "Low"],
  "defaultPriority": "Medium",
  "defaultEditor": "$EDITOR",
  "autoCommit": false
}
```

**Configuration Options:**
- `projectName` - Name of the project
- `statuses` - List of valid status values (customizable per project)
- `priorities` - List of valid priority values
- `defaultPriority` - Default priority for new tasks
- `defaultEditor` - Editor command for opening tasks (defaults to `$EDITOR` environment variable)
- `autoCommit` - If true, automatically commit changes after task operations

# Task Files

Task files are Markdown files with YAML frontmatter containing task metadata. They are the source of truth about a task.

## File Naming Convention

Task files follow the pattern: `[TIMESTAMP]-[slug].md`

- **TIMESTAMP**: Unix timestamp format (e.g., `20260108143022`) serving as the unique ID
- **slug**: Immutable slugified version of the task title (e.g., `create-requirements`)

**Example:** `20260108143022-create-requirements.md`

**Important:** The slug is immutable once created. Even if the Title field is updated, the filename and slug remain unchanged.

## Task References

Tasks reference each other using the `@[slug]` format, where slug is the slugified title from the filename.

**Example:** `@create-requirements`, `@implement-cli`

## Frontmatter Schema

### Required Fields

- `ID` - Unique timestamp-based identifier (auto-generated)
- `Title` - Human-readable task title

### Optional Fields

- `Priority` - Task priority (values: `Critical`, `High`, `Medium`, `Low`)
- `Blocks` - List of task references that are blocked by this task (format: `@slug, @slug`)
- `DependsOn` - List of task references that this task depends on (format: `@slug, @slug`)
- `Tags` - List of tags for categorization

## Sample Task File

```markdown
---
ID: 20260108143022
Title: Create Requirements Document
Priority: High
Blocks: @create-design-document, @implement-cli
Tags: documentation, planning
---

We need to create a comprehensive requirements document for the `repo-tasks` project.

# ToDo

- [x] Write overview
- [x] Describe data model
- [x] Define frontmatter schema
- [ ] Write CLI examples
- [ ] Document AI interaction patterns

# Notes

This should serve as the single source of truth for the project's functionality and design decisions.
```

# CLI Tool

The CLI tool is the primary way to interact with the application.

## Commands

### Initialization
```bash
tasks init
```
Sets up the repository to use repo-tasks. Creates `.repo-tasks/` directory structure and `config.json`.

### Creating Tasks
```bash
tasks new
```
Creates a new task file with auto-generated ID and prompts for Title and other metadata.

### Viewing Tasks
```bash
tasks list [OPTIONS]
```
Lists tasks, excluding "done" status by default, sorted by priority.

**Options:**
- `--status=STATUS` - Filter by status
- `--priority=PRIORITY` - Filter by priority
- `--tag=TAG` - Filter by tag
- `--all` - Include tasks with "done" status
- `--sort=FIELD` - Sort by field (priority, id, created)

```bash
tasks show SLUG
```
Displays task details in the terminal (formatted output of the task file).

### Modifying Tasks
```bash
tasks update SLUG --status=STATUS --priority=PRIORITY
```
Updates task metadata. For modifying Blocks, DependsOn, Tags, or task content, use `tasks open`.

**Examples:**
```bash
tasks update create-requirements --status=in-progress
tasks update create-requirements --priority=Critical
```

```bash
tasks move SLUG STATUS
```
Shorthand for moving a task to a new status.

**Example:**
```bash
tasks move create-requirements done
```

```bash
tasks open SLUG
```
Opens the task file in the default editor for manual editing.

### Searching
```bash
tasks search QUERY
```
Full-text search across all task files (titles, frontmatter, and markdown body).

### Version Control
```bash
tasks save [-m MESSAGE]
```
Commits changes to the `.repo-tasks/` directory.

- If `MESSAGE` is provided, uses it as the commit message
- If no message provided, auto-generates a conventional commit message based on changes
- Auto-generated format: `tasks: created SLUG, moved SLUG to STATUS, updated SLUG`

**Examples:**
```bash
tasks save -m "Reorganized task priorities"
tasks save  # Auto-generates: "tasks: created create-requirements, moved implement-cli to in-progress"
```

## Task Lifecycle

Tasks can move freely between any status without validation. There are no restrictions on status transitions, and tasks do not check dependency states (Blocks/DependsOn) before allowing status changes.

## Git Integration

- **Auto-commit behavior**: Configurable via `autoCommit` in config.json
- **Commit message format**: Uses conventional commit format (`tasks: ...`)
- **Branch linking**: Optional - tasks can include a `Branch` field in frontmatter to link to a git branch

# AI Interactions

LLMs should be able to easily perform CRUD operations on task files. This is a core design goal.

## LLM Capabilities

LLMs should be able to:
1. Read task files and understand the dependency graph
2. Create new tasks with proper frontmatter formatting
3. Update task status and priority
4. Add content to task descriptions
5. Move tasks between status folders
6. Parse and understand task relationships (Blocks/DependsOn)

## Guidelines for LLM-Friendly Design

1. **Consistent Structure**: All task files follow the same frontmatter schema and markdown structure
2. **Clear Examples**: This document provides complete examples for LLMs to reference
3. **Simple Parsing**: YAML frontmatter is standard and easy to parse
4. **Human-Readable References**: Using `@slug` format makes references clear for both humans and LLMs
5. **Self-Documenting**: Task files contain all necessary context within the file

## Example LLM Interactions

**Creating a task:**
```markdown
LLM creates file: `.repo-tasks/tasks/todo/20260108150000-implement-search.md`

---
ID: 20260108150000
Title: Implement Search Functionality
Priority: High
DependsOn: @create-requirements
Tags: feature, cli
---

Implement full-text search across all task files using ripgrep or similar tool.
```

**Updating a task:**
LLM reads the file, parses the frontmatter, updates the Priority field, and writes back.

**Understanding dependencies:**
LLM can traverse the graph by reading task files and following @references in Blocks/DependsOn fields.
