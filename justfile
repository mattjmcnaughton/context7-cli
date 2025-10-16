# Build the project
build:
    cargo build

# Build the project in release mode
build-release:
    cargo build --release

# Run tests (excludes network tests)
test:
    cargo test

# Run tests with output
test-verbose:
    cargo test -- --nocapture

# Run only network tests (requires internet)
test-network:
    RUN_NETWORK_TESTS=1 cargo test -- --ignored

# Run all tests including network tests (requires internet)
test-all:
    RUN_NETWORK_TESTS=1 cargo test -- --include-ignored

# Format code
fmt:
    cargo fmt

# Check formatting without making changes
fmt-check:
    cargo fmt -- --check

# Run clippy linter
lint:
    cargo clippy -- -D warnings

# Run clippy linter and fix issues automatically
lint-fix:
    cargo clippy --fix

# Check the project (fast compile without producing binary)
check:
    cargo check

# Clean build artifacts
clean:
    cargo clean

# Run the CLI
run *ARGS:
    cargo run -- {{ARGS}}

# Run all checks (fmt, lint, test)
ci: fmt-check lint test
