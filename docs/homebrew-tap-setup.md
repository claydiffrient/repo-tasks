# Homebrew Tap Setup Complete

**Repository:** https://github.com/claydiffrient/homebrew-repo-tasks

## What Was Done

✅ **Created GitHub Repository**
- Repository: `claydiffrient/homebrew-repo-tasks`
- Visibility: Public
- Description: "Homebrew tap for repo-tasks"

✅ **Initialized with Formula**
- Added `Formula/repo-tasks.rb` with multi-architecture support
- Added comprehensive README with installation instructions
- Committed and pushed to GitHub

✅ **Tested Tap Installation**
- Successfully tapped with `brew tap claydiffrient/repo-tasks`
- Formula available at `claydiffrient/repo-tasks/repo-tasks`
- Verified formula metadata (version 0.1.0, description, license)

## Current Status

The Homebrew tap is **live and functional** but the formula contains placeholder SHA256 checksums.

**Users can tap the repository now**, but installation will fail until the checksums are updated after the first release (v0.1.0).

## Next Steps (After v0.1.0 Release)

### 1. Update Formula with Real Checksums

After creating the v0.1.0 release:

```bash
# Download all release binaries
curl -LO https://github.com/claydiffrient/repo-tasks/releases/download/v0.1.0/repo-tasks-x86_64-apple-darwin.tar.gz
curl -LO https://github.com/claydiffrient/repo-tasks/releases/download/v0.1.0/repo-tasks-aarch64-apple-darwin.tar.gz
curl -LO https://github.com/claydiffrient/repo-tasks/releases/download/v0.1.0/repo-tasks-x86_64-unknown-linux-gnu.tar.gz
curl -LO https://github.com/claydiffrient/repo-tasks/releases/download/v0.1.0/repo-tasks-aarch64-unknown-linux-gnu.tar.gz

# Calculate SHA256 checksums
shasum -a 256 repo-tasks-x86_64-apple-darwin.tar.gz
shasum -a 256 repo-tasks-aarch64-apple-darwin.tar.gz
shasum -a 256 repo-tasks-x86_64-unknown-linux-gnu.tar.gz
shasum -a 256 repo-tasks-aarch64-unknown-linux-gnu.tar.gz
```

### 2. Update Formula in Tap Repository

```bash
# Clone the tap repository
git clone https://github.com/claydiffrient/homebrew-repo-tasks.git
cd homebrew-repo-tasks

# Edit Formula/repo-tasks.rb
# Replace REPLACE_WITH_ACTUAL_SHA256_FOR_* with real checksums

# Commit and push
git add Formula/repo-tasks.rb
git commit -m "Update checksums for v0.1.0 release"
git push origin main
```

### 3. Test Installation

```bash
brew update
brew install claydiffrient/repo-tasks/repo-tasks
repo-tasks --version
```

## Installation Instructions for Users

Once checksums are updated:

```bash
# Add the tap
brew tap claydiffrient/repo-tasks

# Install repo-tasks
brew install repo-tasks
```

Or directly without tapping:

```bash
brew install claydiffrient/repo-tasks/repo-tasks
```

## Tap Repository Contents

```
homebrew-repo-tasks/
├── Formula/
│   └── repo-tasks.rb    # Formula with multi-arch support
└── README.md            # User documentation
```

## Supported Platforms

- macOS Intel (x86_64)
- macOS Apple Silicon (aarch64)
- Linux x86_64
- Linux ARM64

## Automated Homebrew Updates

The release workflow automatically updates the Homebrew formula when a new release is created.

### Setup Required

Create a GitHub Personal Access Token (PAT) with push access to the tap repository:

1. Go to **GitHub Settings** → **Developer settings** → **Personal access tokens** → **Fine-grained tokens**
2. Click **Generate new token**
3. Configure the token:
   - **Name**: "Homebrew Tap Updates"
   - **Expiration**: Choose appropriate duration
   - **Repository access**: Only select repositories → `homebrew-repo-tasks`
   - **Permissions**:
     - **Contents**: Read and write
4. Generate token and copy it
5. Add to repo secrets:
   - Go to **repo-tasks** repository → **Settings** → **Secrets and variables** → **Actions**
   - Click **New repository secret**
   - **Name**: `HOMEBREW_TAP_TOKEN`
   - **Value**: Paste the token
   - Save

### How It Works

When you create a new release (tag format `v*`):

1. Release workflow builds binaries for all platforms
2. Generates SHA256 checksums
3. Automatically updates `homebrew-repo-tasks/Formula/repo-tasks.rb`:
   - Updates version number
   - Updates download URLs
   - Updates SHA256 checksums for all platforms
4. Commits and pushes to tap repository

**No manual intervention needed!**

### Manual Updates (If Needed)

For future releases if automation fails:

1. Update version number in formula
2. Update URLs to new release tag
3. Calculate new SHA256 checksums
4. Update formula in tap repository
5. Push changes

Users will automatically get updates via `brew update && brew upgrade repo-tasks`.

## Resources

- Tap Repository: https://github.com/claydiffrient/homebrew-repo-tasks
- Main Project: https://github.com/claydiffrient/repo-tasks
- Homebrew Documentation: https://docs.brew.sh/How-to-Create-and-Maintain-a-Tap
