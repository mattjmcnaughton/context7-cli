# Claude Development Guide

This file contains context and guidelines for AI assistants working on this codebase.

## Project Overview

A Rust CLI tool for interacting with the Context7 API to fetch library documentation.

## Development Workflow

- Use `just` for all common tasks (see `justfile`)
- Always run `just ci` before committing
- Follow the contributing guidelines in `CONTRIBUTING.md`

## Rust Best Practices

### Error Handling

- Use `anyhow::Result` for application-level errors
- Use `?` operator for error propagation
- Provide context with `.context()` when errors need explanation

### Code Style

- Follow Rust standard conventions (enforced by `rustfmt`)
- Keep functions focused and single-purpose
- Use descriptive variable names
- Prefer `match` over multiple `if let` chains

### Dependencies

- Minimize external dependencies
- Prefer well-maintained crates from trusted authors
- Keep `Cargo.toml` organized and commented when necessary

### Testing

- Write unit tests for pure functions
- Write integration tests for CLI commands
- Use `cargo test` to run all tests
- Mock external API calls in tests

### CLI Design

- Use `clap` for argument parsing with derive macros
- Provide helpful error messages
- Follow Unix conventions (exit codes, stdout/stderr)
- Keep commands simple and composable

## Project Structure

```
src/
├── main.rs          # Entry point and CLI setup
└── ...              # Additional modules as project grows
```

## Common Tasks

See `justfile` for available commands. Most important:
- `just build` - Compile the project
- `just test` - Run tests
- `just ci` - Run all checks (fmt, lint, test)
