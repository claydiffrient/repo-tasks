---
ID: "20260109231636"
Title: Fix CI failures on GitHub Actions
Priority: Critical
Tags:
  - ci
  - github-actions
  - bugs
---

CI is failing with multiple issues:

## 1. Clippy Error (blocking)

**Location:** `src/utils/errors.rs:1:20`

**Issue:** Unused import 'Context' from anyhow

**Fix:** Remove 'Context' from the import statement on line 1

```rust
// Current:
use anyhow::{bail, Context, Result};

// Should be:
use anyhow::{bail, Result};
```

## 2. Deprecated GitHub Actions (blocking)

**Issue:** Using deprecated `actions/upload-artifact@v3`

**Affects:**
- Build Release jobs (all 4 targets)
- Benchmark job

**Fix:** Upgrade to `actions/upload-artifact@v4` in `.github/workflows/ci.yml`

## 3. Deprecated Checkout Action (nice-to-have)

**Issue:** Using `actions/checkout@v3`

**Fix:** Upgrade to `actions/checkout@v4` in `.github/workflows/ci.yml`

## 4. Deprecated Cache Action (nice-to-have)

**Issue:** Using `actions/cache@v3`

**Fix:** Upgrade to `actions/cache@v4` in `.github/workflows/ci.yml`

## Priority

The clippy error must be fixed first as it's preventing compilation. The deprecated actions are also blocking the build artifacts from being created.

## Reference

- Failed run: https://github.com/claydiffrient/repo-tasks/actions/runs/20855719959
- Deprecation notice: https://github.blog/changelog/2024-04-16-deprecation-notice-v3-of-the-artifact-actions/
