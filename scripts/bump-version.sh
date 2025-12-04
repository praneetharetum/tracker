#!/bin/bash
set -e

# Version bump script for Tauri app
# Syncs version across package.json, Cargo.toml, and tauri.conf.json
#
# Usage: ./scripts/bump-version.sh <version>
# Example: ./scripts/bump-version.sh 0.2.0

if [ -z "$1" ]; then
    echo "Usage: $0 <version>"
    echo "Example: $0 0.2.0"
    exit 1
fi

VERSION="$1"

# Validate version format (semver)
if ! [[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "Error: Version must be in semver format (e.g., 0.2.0)"
    exit 1
fi

echo "Bumping version to $VERSION..."

# Update package.json
if [ -f "package.json" ]; then
    if command -v jq &> /dev/null; then
        jq ".version = \"$VERSION\"" package.json > package.json.tmp && mv package.json.tmp package.json
    else
        sed -i.bak "s/\"version\": \"[^\"]*\"/\"version\": \"$VERSION\"/" package.json && rm -f package.json.bak
    fi
    echo "✓ Updated package.json"
fi

# Update src-tauri/Cargo.toml
if [ -f "src-tauri/Cargo.toml" ]; then
    sed -i.bak "s/^version = \"[^\"]*\"/version = \"$VERSION\"/" src-tauri/Cargo.toml && rm -f src-tauri/Cargo.toml.bak
    echo "✓ Updated src-tauri/Cargo.toml"
fi

# Update src-tauri/tauri.conf.json
if [ -f "src-tauri/tauri.conf.json" ]; then
    if command -v jq &> /dev/null; then
        jq ".version = \"$VERSION\"" src-tauri/tauri.conf.json > src-tauri/tauri.conf.json.tmp && mv src-tauri/tauri.conf.json.tmp src-tauri/tauri.conf.json
    else
        sed -i.bak "s/\"version\": \"[^\"]*\"/\"version\": \"$VERSION\"/" src-tauri/tauri.conf.json && rm -f src-tauri/tauri.conf.json.bak
    fi
    echo "✓ Updated src-tauri/tauri.conf.json"
fi

echo ""
echo "Version bumped to $VERSION"
echo ""
echo "Next steps:"
echo "  1. Review the changes: git diff"
echo "  2. Commit: git commit -am \"chore: bump version to $VERSION\""
echo "  3. Tag: git tag v$VERSION"
echo "  4. Push: git push && git push --tags"
