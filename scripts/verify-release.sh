#!/usr/bin/env bash
#
# verify-release.sh - Verify repo-tasks release checksum
#
# Usage:
#   ./verify-release.sh <version> <artifact>
#
# Examples:
#   ./verify-release.sh v0.1.0 repo-tasks-x86_64-apple-darwin.tar.gz
#   ./verify-release.sh v0.1.0 repo-tasks-x86_64-pc-windows-msvc.zip
#

set -euo pipefail

VERSION="${1:-}"
ARTIFACT="${2:-}"

if [ -z "$VERSION" ] || [ -z "$ARTIFACT" ]; then
    echo "Usage: $0 <version> <artifact>"
    echo ""
    echo "Examples:"
    echo "  $0 v0.1.0 repo-tasks-x86_64-apple-darwin.tar.gz"
    echo "  $0 v0.1.0 repo-tasks-x86_64-pc-windows-msvc.zip"
    exit 1
fi

REPO="claydiffrient/repo-tasks"
CHECKSUMS_URL="https://github.com/${REPO}/releases/download/${VERSION}/checksums.txt"

echo "Verifying ${ARTIFACT} from ${VERSION}..."
echo ""

# Check if artifact exists
if [ ! -f "$ARTIFACT" ]; then
    echo "Error: File not found: ${ARTIFACT}"
    echo "Please download the artifact first:"
    echo "  curl -LO https://github.com/${REPO}/releases/download/${VERSION}/${ARTIFACT}"
    exit 1
fi

# Download checksums file
echo "Downloading checksums..."
if ! curl -fsSL "$CHECKSUMS_URL" -o checksums.txt; then
    echo "Error: Failed to download checksums.txt"
    echo "URL: ${CHECKSUMS_URL}"
    exit 1
fi

# Extract expected checksum for this artifact
EXPECTED=$(grep "$ARTIFACT" checksums.txt | awk '{print $1}')

if [ -z "$EXPECTED" ]; then
    echo "Error: No checksum found for ${ARTIFACT} in checksums.txt"
    exit 1
fi

echo "Expected: ${EXPECTED}"

# Calculate actual checksum
if command -v shasum >/dev/null 2>&1; then
    ACTUAL=$(shasum -a 256 "$ARTIFACT" | awk '{print $1}')
elif command -v sha256sum >/dev/null 2>&1; then
    ACTUAL=$(sha256sum "$ARTIFACT" | awk '{print $1}')
else
    echo "Error: Neither shasum nor sha256sum found"
    exit 1
fi

echo "Actual:   ${ACTUAL}"
echo ""

# Compare checksums
if [ "$EXPECTED" = "$ACTUAL" ]; then
    echo "✓ Checksum verified successfully!"
    echo "  ${ARTIFACT} is authentic"
    exit 0
else
    echo "✗ Checksum mismatch!"
    echo "  ${ARTIFACT} may be corrupted or tampered with"
    echo "  Please re-download the file and try again"
    exit 1
fi
