#!/usr/bin/env bash
# Ensures tests and formatting passes, then creates a PR.
# The first argument is the PR title, the second is optional, and if
# passed is used as the PR body.
set -euo pipefail

title="$1"
body="${2:-}"

cargo test
cargo fmt

branch="$(git branch --show-current)"
git push -u origin "$branch"

gh pr create \
  --base "main" \
  --head "$branch" \
  --title "$title" \
  --body "$body"
