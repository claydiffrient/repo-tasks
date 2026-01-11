---
ID: '20260111143010'
Title: Add config management MCP commands
Priority: Medium
Tags:
- mcp
- enhancement
- config
---

Add MCP tools for managing the config.json file:
- get_config: Read current configuration
- update_config: Update configuration settings (statuses, priorities, auto_commit, etc.)
- add_status: Add a new status to the workflow
- remove_status: Remove a status from the workflow
- add_priority: Add a new priority level
- remove_priority: Remove a priority level

These tools will allow LLMs to customize the task management workflow through natural language.

## Implementation Plan

**Complexity:** ⭐⭐⭐ Complex (hybrid approach)
**Estimated Time:** ~10-15 hours
**Agent ID:** ab17b93
**Dependencies:** JSON Schema task (20260111143141) for best results

### Recommended Approach: Hybrid

1. **Add Rust CLI commands** for config management
2. **MCP server wraps CLI** for consistency
3. **Direct JSON read** for get_config only (performance)

**Rationale:**
- Maintains architectural consistency
- CLI benefits all users, not just MCP
- Type-safe validation in Rust
- Single source of truth

### 8 New Tools

1. `get_config` - Read current configuration
2. `set_config` - Update settings (project_name, auto_commit, output_format)
3. `add_status` - Add workflow status with optional position
4. `remove_status` - Remove status (with safety checks)
5. `add_priority` - Add priority level with optional position
6. `remove_priority` - Remove priority level
7. `list_statuses` - List all statuses
8. `list_priorities` - List all priorities

### Phase 1: Rust Config Model Enhancements

**File:** `src/models/config.rs`

Add methods:
```rust
impl Config {
    pub fn validate(&self) -> Result<()>
    pub fn add_status(&mut self, status: String, position: Option<usize>) -> Result<()>
    pub fn remove_status(&mut self, status: &str) -> Result<()>
    pub fn add_priority(&mut self, priority: String, position: Option<usize>) -> Result<()>
    pub fn remove_priority(&mut self, priority: &str) -> Result<()>
    pub fn set_value(&mut self, key: &str, value: &str) -> Result<()>
}
```

### Phase 2: Rust CLI Commands

**New File:** `src/commands/config.rs` (~300-400 lines)

Implement 8 functions:
- `get()` - Output JSON
- `set(key, value)` - Validate and update
- `add_status(status, position)` - Add with uniqueness check
- `remove_status(status)` - Safety check for tasks in status
- `add_priority(priority, position)` - Add with uniqueness check
- `remove_priority(priority)` - Remove priority
- `list_statuses()` - List all
- `list_priorities()` - List all

### Phase 3: CLI Integration

**File:** `src/main.rs`

Add ConfigSubcommand enum with 8 variants:
```rust
#[derive(Subcommand)]
enum ConfigSubcommand {
    Get,
    Set { key: String, value: String },
    AddStatus { status: String, #[arg(short, long)] position: Option<usize> },
    RemoveStatus { status: String },
    AddPriority { priority: String, #[arg(short, long)] position: Option<usize> },
    RemovePriority { priority: String },
    ListStatuses,
    ListPriorities,
}
```

Add to Commands enum:
```rust
Config {
    #[command(subcommand)]
    subcommand: ConfigSubcommand,
}
```

### Phase 4: MCP Server Implementation

**File:** `mcp-server/src/index.ts`

Add helper:
```typescript
function readConfigDirect(): any {
  const configPath = resolve(process.cwd(), ".repo-tasks/config.json");
  const content = readFileSync(configPath, "utf-8");
  return JSON.parse(content);
}
```

Add 8 tool definitions to TOOLS array.

Add 8 case handlers:
- `get_config` - Use direct read with CLI fallback
- `set_config` - Execute `tasks config set key value`
- `add_status` - Execute `tasks config add-status status [--position N]`
- `remove_status` - Execute `tasks config remove-status status`
- `add_priority` - Execute `tasks config add-priority priority [--position N]`
- `remove_priority` - Execute `tasks config remove-priority priority`
- `list_statuses` - Execute `tasks config list-statuses`
- `list_priorities` - Execute `tasks config list-priorities`

### Validation Rules

- At least 1 status and 1 priority required
- No duplicate statuses or priorities
- Status names: `^[a-z][a-z0-9-]*$`
- Can't remove status with tasks in it

### Build Sequence

- [ ] Phase 1: Add Config model methods with tests
- [ ] Phase 2: Create config.rs with 8 functions
- [ ] Phase 3: Integrate into main.rs CLI
- [ ] Test CLI commands manually
- [ ] Phase 4: Add MCP tools and handlers
- [ ] Build and test MCP server
- [ ] Integration testing with Claude Desktop
- [ ] Documentation updates

### Critical Safety Checks

**remove_status:** Check if tasks exist in that status before allowing removal
**Atomicity:** All config changes are load → validate → write (fail-safe)
**Uniqueness:** Validate no duplicate statuses/priorities