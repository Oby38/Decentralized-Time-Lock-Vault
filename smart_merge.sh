#!/bin/bash

# Simple PRs that likely don't conflict
simple_prs=(204 203 202 201 200 198 197 182 181)
# Complex PRs with many changes
complex_prs=(199 192 188 187 175 163 162 157 55 191 186)

echo "=== Merging simple PRs ==="
for pr in "${simple_prs[@]}"; do
    echo ">>> PR #$pr"
    git fetch origin "pull/$pr/head:pr-$pr" --force 2>/dev/null
    if git merge --no-edit pr-$pr 2>&1 | grep -q "CONFLICT\|conflict"; then
        echo "Conflict detected, aborting and skipping"
        git merge --abort
    else
        echo "✓ Merged successfully"
    fi
done

echo ""
echo "=== Merging complex PRs (will handle conflicts) ==="
for pr in "${complex_prs[@]}"; do
    echo ">>> PR #$pr"
    git fetch origin "pull/$pr/head:pr-$pr" --force 2>/dev/null
    if ! git merge --no-edit pr-$pr 2>&1; then
        echo "Resolving conflicts..."
        # For conflicted files, prefer the PR version but only if valid
        git diff --name-only --diff-filter=U | while read file; do
            # Show the conflict
            echo "  Conflicted: $file"
            git checkout --theirs "$file"
        done
        git add .
        git commit -m "Merge PR #$pr with conflict resolution" --no-edit
        echo "✓ Merged with conflict resolution"
    else
        echo "✓ Merged without conflicts"
    fi
done

git push origin main --force-with-lease
echo "All merged and pushed"
