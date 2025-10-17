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

This project follows a clean separation between pure business logic and I/O operations to maximize unit test coverage.

#### Architecture for Testability

**Core Module** (`src/core/`): Pure business logic with no I/O
- `sorting.rs` - Sort search results by various fields
- `validation.rs` - Validate inputs (e.g., non-empty results)
- `formatting.rs` - Format output (JSON, ID-only)

**Commands Module** (`src/commands/`): Thin wrappers around I/O + core logic
- Handle API calls, CLI arguments, and output
- Delegate business logic to core functions
- Keep I/O and logic separate for easier testing

**Models Module** (`src/models/`): Data structures with serialization tests
- Test JSON deserialization/serialization
- Verify field name mappings (camelCase ↔ snake_case)

#### Unit Testing Guidelines

**What to Unit Test:**
- Pure functions in `src/core/` (no external dependencies)
- Data model serialization in `src/models/`
- Business logic: sorting, validation, formatting
- Edge cases: empty inputs, None values, invalid data

**Unit Test Best Practices:**
- Place tests in `#[cfg(test)]` modules at bottom of file
- Use descriptive test names: `test_sort_by_stars_descending`
- Test one behavior per test function
- Use `mock_json` variable name for JSON test data
- Create helper functions to reduce test boilerplate
- Tests should run in milliseconds (no I/O)

**Example Unit Test:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_by_stars_descending() {
        let results = vec![
            create_test_result("low", Some(10)),
            create_test_result("high", Some(100)),
        ];

        let sorted = sort_search_results(results, SortField::Stars);

        assert_eq!(sorted[0].id, "high");
        assert_eq!(sorted[1].id, "low");
    }
}
```

#### Integration and E2E Testing

Integration and e2e tests live in the `tests/` directory and should be marked as `#[ignore]` to require explicit opt-in.

**Integration Tests** (`tests/test_*_integration.rs`):
- Test a small number of components working together
- Test a single component interacting with external dependencies (e.g., API client making real HTTP calls)
- Focus on verifying component interactions and external dependency behavior
- Example: Testing that the Context7 client correctly calls the API and parses responses

**End-to-End (E2E) Tests** (`tests/test_*_e2e.rs`):
- Test high-level, complete user workflows from start to finish
- Use `assert_cmd` to test the full CLI experience
- Verify command-line arguments, exit codes, stdout/stderr output
- Example: Running `context7-cli search rust --sort stars` and verifying the complete formatted output

**Best Practices:**
- Both integration and e2e tests may interact with external services
- All tests requiring network I/O should be marked `#[ignore]`
- Use environment variable `RUN_NETWORK_TESTS=1` to opt-in to running these tests
- Keep e2e tests focused on critical user journeys
- Use integration tests to verify component interactions and external API behavior

#### Running Tests

```bash
# Run all unit tests (fast, no network I/O)
cargo test

# Run CI checks (format, lint, unit tests only)
just ci

# Run integration and e2e tests (requires network)
RUN_NETWORK_TESTS=1 cargo test -- --ignored

# Run all tests including integration and e2e (requires network)
RUN_NETWORK_TESTS=1 cargo test -- --include-ignored
```

#### Test Coverage Goals

- **Core module**: >80% coverage (pure logic is easy to test)
- **Commands module**: Integration tests only (thin I/O wrappers)
- **Models module**: Test all serialization scenarios

### CLI Design

- Use `clap` for argument parsing with derive macros
- Provide helpful error messages
- Follow Unix conventions (exit codes, stdout/stderr)
- Keep commands simple and composable

## Project Structure

```
src/
├── main.rs          # Entry point and CLI setup
├── cli.rs           # CLI argument definitions
├── clients/         # External API clients
│   └── context7.rs  # Context7 API client
├── commands/        # CLI command implementations (I/O heavy)
│   ├── search.rs    # Search command
│   ├── get_docs.rs  # Get documentation command
│   └── lucky.rs     # "I'm feeling lucky" command
├── core/            # Pure business logic (highly testable)
│   ├── sorting.rs   # Sort search results
│   ├── validation.rs # Input validation
│   └── formatting.rs # Output formatting
└── models/          # Data structures
    └── search.rs    # Search-related types
tests/
└── e2e.rs          # End-to-end integration tests
```

## Common Tasks

See `justfile` for available commands. Most important:
- `just build` - Compile the project
- `just test` - Run tests
- `just ci` - Run all checks (fmt, lint, test)
