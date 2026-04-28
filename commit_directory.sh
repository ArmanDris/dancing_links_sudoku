#!/usr/bin/env bash
# Adds and commits all changes in the working directory. The first argument is
# the commit message.
if [$# -eq 0 ]; then
  echo "Usage $0 <commit message>"
  exit 1
fi

git add .
git commit -m "$1"
