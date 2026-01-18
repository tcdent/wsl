#!/bin/bash
# Script to create and push the main branch

set -e

echo "Creating main branch from WSL-spec-draft content..."

# Ensure we're in the repository root
cd "$(git rev-parse --show-toplevel)"

# Check if we have a remote configured
if ! git remote get-url origin >/dev/null 2>&1; then
    echo "Error: No 'origin' remote configured"
    exit 1
fi

# Create main branch from the WSL-spec-draft commit (70f0824)
if git show-ref --verify --quiet refs/heads/main; then
    echo "Main branch already exists locally"
else
    if git branch main 70f0824d741798a0dfb08f6b946acace95031a2b; then
        echo "Main branch created locally"
    else
        echo "Error: Failed to create main branch"
        exit 1
    fi
fi

# Push the main branch to origin
echo "Pushing main branch to origin..."
if git push origin main; then
    echo "Done! Main branch has been created and pushed."
    echo "You can verify it at: https://github.com/tcdent/wsl/tree/main"
else
    echo "Error: Failed to push main branch. Please check your permissions."
    exit 1
fi
