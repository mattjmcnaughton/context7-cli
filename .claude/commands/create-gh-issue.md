# create-gh-issue

Create a GitHub issue using the `gh` CLI with consistent formatting.

## Format

**Title**: Prefix with `feat:`, `bug:`, or `spike:`

**Body structure**:
```
## Description
[What needs to be done]

## Context
[Why this matters - optional]

## Tasks
- [ ] [Key steps - optional]
```

## Usage

After drafting the issue content, create it with:
```bash
# Write the issue body to a temp file
cat > /tmp/issue_body.txt <<'EOF'
## Description
...

## Context
...

## Tasks
- [ ] ...
EOF

# Create the issue using the temp file
gh issue create --title "prefix: title" --body "$(cat /tmp/issue_body.txt)"
```

**Important**: Always use single quotes in the HEREDOC (`<<'EOF'`) to prevent shell expansion and ensure special characters are treated literally.

Keep descriptions brief and tasks high-level. Add details in comments as work progresses.
