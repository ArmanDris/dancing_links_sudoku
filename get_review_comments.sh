#!/usr/bin/env bash
# Fetches PR comments associated with the current branch
set -euo pipefail
pr_number=$(gh pr view --json number -q .number)
gh api "repos/ArmanDris/dancing_links_sudoku/pulls/$pr_number/comments"
