---
ID: '20260109231636'
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

## Update (2026-01-10)

### ✅ Fixed in commit 78af672

All original issues have been resolved:

1. ✅ **Clippy Error** - Removed unused `Context` import and fixed all clippy warnings
2. ✅ **Deprecated Actions** - Upgraded all actions to v4:
   - `actions/checkout@v3` → `v4`
   - `actions/cache@v3` → `v4`
   - `actions/upload-artifact@v3` → `v4`

### CI Results (run 20874302650)

**Passing Jobs:**
- ✅ Lint (clippy + rustfmt)
- ✅ Test (ubuntu-latest, macos-latest, windows-latest)
- ✅ Code Coverage
- ✅ Benchmark

**Still Failing:**
- ❌ Build Release jobs (all 4 cross-platform targets)
  - Error: `openssl-sys v0.9.111` compilation failure
  - Cause: Cross-compilation requires OpenSSL, not available in CI environment
  - **Note:** This is a separate issue related to cross-platform builds, tracked in task #20260109120001

### Conclusion

The original CI failures (clippy errors and deprecated actions) are **fully resolved**. The build release failures are a different issue related to cross-platform compilation dependencies and should be addressed as part of the cross-platform release builds task.