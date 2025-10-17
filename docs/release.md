# Release and Publish Process

This document describes how releases and binary publishing work for context7-cli.

## Overview

Releases are fully automated using [semantic-release](https://github.com/semantic-release/semantic-release) and GitHub Actions. When changes are merged to `main`, the system automatically:

1. Analyzes commit messages to determine the version bump
2. Updates the version in `Cargo.toml`
3. Generates a changelog
4. Creates a Git tag and GitHub release
5. Builds and uploads binaries for all supported platforms

## How It Works

### 1. Commit Message Convention

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

- `fix:` - Patch release (1.0.x)
- `feat:` - Minor release (1.x.0)
- `BREAKING CHANGE:` or `feat!:` / `fix!:` - Major release (x.0.0)
- `chore:`, `docs:`, `test:` - No release

Example:
```
feat: add support for library version filtering

This adds a new --version flag to filter documentation by version.
```

### 2. Automated Workflow

When CI passes on the `main` branch:

1. **Semantic Release Job** (`release.yml`):
   - Runs `semantic-release` to analyze commits
   - If a release is warranted:
     - Updates `Cargo.toml` version
     - Generates/updates `CHANGELOG.md`
     - Creates a commit (tagged with `[skip ci]`)
     - Creates a Git tag (e.g., `v1.2.3`)
     - Creates a GitHub release
     - Outputs the new version via `$GITHUB_OUTPUT`

2. **Build Binaries Job** (`release.yml`):
   - Only runs if a new release was published
   - Builds binaries in parallel for:
     - Linux (x86_64)
     - macOS Intel (x86_64)
     - macOS Apple Silicon (aarch64)
   - Uploads both compressed (`.tar.gz`) and uncompressed binaries to the GitHub release

### 3. Configuration Files

- `.releaserc.json` - Semantic-release configuration
  - Defines plugins and their order
  - Configures version bumping in `Cargo.toml`
  - Sets up changelog generation
  - Handles Git commits and GitHub releases

- `.github/workflows/release.yml` - Release workflow
  - Triggers on successful CI runs
  - Runs semantic-release
  - Builds and publishes binaries

## Manual Intervention

In most cases, no manual intervention is needed. However:

### Forcing a Release Type

Add to commit message body:
```
feat: add new feature

BREAKING CHANGE: This changes the API contract
```

### Skipping CI

Add `[skip ci]` to the commit message (semantic-release does this automatically for release commits).

### Creating a Pre-release

Pre-releases are not currently configured, but can be added by updating the `branches` configuration in `.releaserc.json`.

## Supported Platforms

Binaries are automatically built for:
- `x86_64-unknown-linux-gnu` (Linux 64-bit)
- `x86_64-apple-darwin` (macOS Intel)
- `aarch64-apple-darwin` (macOS Apple Silicon)

To add more platforms, update the matrix in `.github/workflows/release.yml`.
