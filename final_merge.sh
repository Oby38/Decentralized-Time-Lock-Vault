#!/bin/bash

prs=(199 192 188 187 175 163 162 157 55)

for pr in "${prs[@]}"; do
    echo "================================"
    echo "Merging PR #$pr into main"
    echo "================================"
    
    # Fetch the latest PR branch
    git fetch origin "pull/$pr/head:pr-$pr" --force 2>/dev/null
    
    # Try to merge into main
    if ! git merge pr-$pr --no-edit 2>&1 | grep -q "CONFLICT"; then
        echo "✓ PR #$pr merged successfully"
    else
        echo "⚠ Conflicts in PR #$pr, resolving..."
        # Show conflicts
        git diff --name-only --diff-filter=U
        # Keep PR changes for all files
        git checkout --theirs .
        git add .
        git commit -m "Merge PR #$pr - resolved conflicts" --no-edit
        echo "✓ Conflicts resolved"
    fi
done

# Push all changes to main
echo "================================"
echo "Pushing to main"
echo "================================"
git push origin main
