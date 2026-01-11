---
ID: '20260111143141'
Title: Create JSON schema for config.json
Priority: Medium
Tags:
- config
- schema
- validation
- docs
---

Create a JSON schema file (config.schema.json) that defines the structure and validation rules for .repo-tasks/config.json. This will:
- Enable IDE autocomplete and validation for config files
- Document all available configuration options
- Provide type checking for programmatic config updates
- Serve as foundation for config management MCP commands

The schema should define:
- project_name (string, required)
- statuses (array of strings, required, min 1 item)
- priorities (array of strings, required, min 1 item)
- auto_commit (boolean, optional, default false)
- Any future config options

Add schema validation to the init and config update commands.

## Implementation Plan

**Complexity:** ⭐⭐ Moderate (multi-layer approach)
**Estimated Time:** ~3-4 hours
**Agent ID:** a2eb265

### Multi-Tier Validation Strategy

1. **IDE-level:** JSON Schema enables autocomplete and inline validation
2. **Runtime-level:** Programmatic validation provides clear error messages
3. **Optional strict-level:** Feature-gated jsonschema validation for CI/CD

### Phase 1: Create JSON Schema File

**File:** `config.schema.json` (repository root)

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://github.com/claydiffrient/repo-tasks/config.schema.json",
  "title": "Repo Tasks Configuration",
  "description": "Configuration file for repo-tasks CLI",
  "type": "object",
  "required": ["project_name", "statuses", "priorities"],
  "properties": {
    "project_name": {
      "type": "string",
      "description": "Display name for the project",
      "minLength": 1,
      "maxLength": 100
    },
    "statuses": {
      "type": "array",
      "description": "Workflow status values for tasks",
      "items": {
        "type": "string",
        "pattern": "^[a-z][a-z0-9-]*$"
      },
      "minItems": 1,
      "uniqueItems": true
    },
    "priorities": {
      "type": "array",
      "description": "Priority levels for tasks",
      "items": {
        "type": "string",
        "minLength": 1
      },
      "minItems": 1,
      "uniqueItems": true
    },
    "auto_commit": {
      "type": "boolean",
      "description": "Automatically commit changes after operations",
      "default": false
    },
    "output_format": {
      "type": "string",
      "description": "Default output format for list command",
      "enum": ["table", "json", "yaml"],
      "default": "table"
    }
  },
  "additionalProperties": false
}
```

### Phase 2: Add Programmatic Validation

**File:** `src/models/config.rs`

Add methods to Config impl:
```rust
pub fn validate(&self) -> Result<()>
fn validate_project_name(&self) -> Result<()>
fn validate_statuses(&self) -> Result<()>
fn validate_priorities(&self) -> Result<()>
```

Integrate into:
- `Config::load()` - Call validate() after deserialization
- `Config::write()` - Call validate() before serialization

### Phase 3: Update Init Command

**File:** `src/commands/init.rs`

- Embed schema using `include_str!` macro
- Add `$schema` field to generated config.json
- Optionally write schema file to `.repo-tasks/`

### Phase 4: Optional JSON Schema Validation (Feature Flag)

**File:** `Cargo.toml`
```toml
[features]
schema-validation = ["jsonschema"]

[dependencies]
jsonschema = { version = "0.26", optional = true }
```

**New File:** `src/models/schema_validator.rs`

### Build Sequence
- [ ] Create config.schema.json
- [ ] Add validation methods to Config
- [ ] Integrate validation into load/write
- [ ] Write unit tests for validation
- [ ] Update init command with $schema
- [ ] Test with VSCode IDE validation
- [ ] Add feature-gated jsonschema validation (optional)