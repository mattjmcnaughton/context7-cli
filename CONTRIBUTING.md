# Contributing

## Prerequisites

- Rust (latest stable version)
- [just](https://github.com/casey/just) command runner

## Development

```bash
# Build the project
just build

# Run tests
just test

# Format code
just fmt

# Run linter
just lint

# Run all checks before committing
just ci
```

## Running

```bash
# Run the CLI with arguments
just run <command> [args]
```

## Pull Requests

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run `just ci` to ensure all checks pass
5. Submit a pull request

## Code Style

- Follow standard Rust conventions
- Run `just fmt` before committing
- Ensure `just lint` passes without warnings
