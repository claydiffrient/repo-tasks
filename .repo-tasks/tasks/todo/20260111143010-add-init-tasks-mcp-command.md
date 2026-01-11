---
ID: '20260111143010'
Title: Add init_tasks MCP command
Priority: High
Tags:
- mcp
- enhancement
- init
---

Add an MCP tool called 'init_tasks' that wraps the 'tasks init' command. This allows LLMs to initialize a new repo-tasks repository through the MCP server. Should accept optional project_name parameter and handle both interactive and non-interactive modes.

## Implementation Plan

**Complexity:** ⭐ Simple (2 insertions, 1 file)
**Estimated Time:** ~30 minutes
**Agent ID:** ada3964

### Changes Required

**File:** `mcp-server/src/index.ts`

**Change 1: Add Tool Definition** (after line ~227)
```typescript
{
  name: "init_tasks",
  description:
    "Initialize a new repo-tasks repository in the current directory. Creates .repo-tasks/ directory structure with config.json and task directories (todo, in-progress, testing, done). Must be run in a git repository that doesn't already have repo-tasks initialized.",
  inputSchema: {
    type: "object",
    properties: {
      project_name: {
        type: "string",
        description: "Project name for the repository (defaults to current directory name if not provided)",
      },
    },
  },
},
```

**Change 2: Add Tool Handler** (after line ~332)
```typescript
case "init_tasks": {
  const cmdArgs = ["init"];
  if (args?.project_name) cmdArgs.push("--project-name", args.project_name as string);

  const output = executeCommand(cmdArgs);
  return {
    content: [{ type: "text", text: output }],
  };
}
```

### Verification Steps
- [ ] TypeScript compiles without errors
- [ ] Tool appears in MCP tools list (10 tools total)
- [ ] Test with project_name parameter
- [ ] Test without project_name parameter
- [ ] Test error case (already initialized directory)