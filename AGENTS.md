### On task completion

1. Use the script `commit_changes.sh <commit-message>` to commit add then commit
the changes to git.

2. Use the script: `test_then_publish_pr.sh <title> [body]` to publish the 
changes.
 - The title should describe the changes in 4-8 words
 - Only include a body if there are non-obvious changes that should be
   specified, otherwise it can be left blank
 - The script will fail if `cargo test` or `cargo fmt` fails
