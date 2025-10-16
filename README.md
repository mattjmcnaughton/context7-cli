# context7-cli

A command-line interface for accessing the [Context7](https://context7.com) API, enabling developers and LLMs to search for libraries and retrieve documentation programmatically.

## Overview

Context7 provides a comprehensive database of library documentation. This CLI tool allows you to:
- Search for libraries across multiple programming languages and frameworks
- Retrieve detailed documentation for specific libraries
- Auto-select top search results with the "lucky" command
- Sort and filter search results based on various metrics
- Integrate Context7 data into your development workflows

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70 or later)
- Cargo (comes with Rust)

### Building from Source

```bash
git clone https://github.com/mattjmcnaughton/context7-cli.git
cd context7-cli
cargo build --release
```

The compiled binary will be available at `target/release/context7-cli`.

### Running Without Installation

You can run the CLI directly with Cargo:

```bash
cargo run -- <command> [options]
```

## Quick Start

Auto-fetch documentation for the most popular result:
```bash
context7-cli lucky "fastapi"
```

Search for a library:
```bash
context7-cli search "fastapi"
```

Get documentation for a specific library:
```bash
context7-cli get-docs "/fastapi/fastapi"
```

## Commands

### `lucky`

Search and automatically fetch documentation for the most popular (most starred) result. This is the fastest way to get documentation when you know what you're looking for.

**Usage:**
```bash
context7-cli lucky <query>
```

**Arguments:**
- `<query>` - The search query string

**Examples:**

Get FastAPI documentation instantly:
```bash
context7-cli lucky "fastapi"
```

Get React documentation:
```bash
context7-cli lucky "react"
```

**How it works:**
1. Searches Context7 for your query
2. Sorts results by star count (popularity)
3. Automatically fetches and displays documentation for the top result
4. Returns an error if no results are found

### `search`

Search the Context7 API for libraries matching a query string.

**Usage:**
```bash
context7-cli search <query> [OPTIONS]
```

**Arguments:**
- `<query>` - The search query string

**Options:**
- `--sort-by <FIELD>` - Field to sort results by (default: "stars")
  - Valid options: `stars`, `totalPages`, `totalSnippets`, `totalTokens`, `trustScore`
- `--limit <NUMBER>` - Limit the number of results returned
- `--id-only` - Output only the ID field (one per line)

**Examples:**

Search for FastAPI libraries:
```bash
context7-cli search "fastapi"
```

Search and sort by trust score:
```bash
context7-cli search "react" --sort-by trustScore
```

Limit results to top 5:
```bash
context7-cli search "python web framework" --limit 5
```

Get only library IDs for piping to other commands:
```bash
context7-cli search "tensorflow" --id-only
```

**Output Format:**

By default, `search` returns JSON with detailed information about each library:
```json
[
  {
    "id": "/fastapi/fastapi",
    "title": "FastAPI",
    "description": "FastAPI framework, high performance, easy to learn...",
    "stars": 75000,
    "trustScore": 9.5,
    "totalPages": 150,
    "totalSnippets": 1200,
    "totalTokens": 500000,
    "state": "active",
    "branch": "master",
    "lastUpdateDate": "2025-01-15"
  }
]
```

### `get-docs`

Retrieve documentation for a specific library by its Context7 ID.

**Usage:**
```bash
context7-cli get-docs <id>
```

**Arguments:**
- `<id>` - Library ID (e.g., "/fastapi/fastapi" or "fastapi/fastapi")

**Examples:**

Get FastAPI documentation:
```bash
context7-cli get-docs "/fastapi/fastapi"
```

Get documentation without leading slash:
```bash
context7-cli get-docs "fastapi/fastapi"
```

**Output Format:**

Returns the raw documentation content from Context7.

## LLM Integration

LLMs can use this CLI to retrieve up-to-date library documentation:

```bash
# Quick documentation lookup with lucky
context7-cli lucky "fastapi"

# Get current documentation for a specific library version
context7-cli get-docs "/vercel/next.js"

# Search for alternatives and compare
context7-cli search "react framework" --sort-by trustScore --limit 5
```

The `lucky` command is particularly useful for LLMs when they need quick access to documentation without making multiple API calls.

## Development

### Prerequisites

- Rust (latest stable version)
- [just](https://github.com/casey/just) command runner

### Common Commands

```bash
# Build the project
just build

# Run tests
just test

# Run all checks (format, lint, test)
just ci

# Run the CLI with arguments
just run search "fastapi"
```

For detailed development instructions, see [CONTRIBUTING.md](CONTRIBUTING.md).

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details on:
- Setting up the development environment
- Running tests
- Submitting pull requests
- Code style guidelines

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Resources

- [Context7 Website](https://context7.com)
- [Issue Tracker](https://github.com/mattjmcnaughton/context7-cli/issues)
