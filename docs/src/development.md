# Development

This chapter is for people building or contributing to `obsidian-logging` itself, rather than just
using the CLI.

## Quick start

```bash
git clone https://github.com/ljantzen/obsidian-logging
cd obsidian-logging
just build      # Build everything
just test       # Run all tests
just check      # Run fmt, clippy, and tests
```

## Project layout

The project is a Cargo workspace with two crates:

- `lib/` — `obsidian-logging`, the core library (logging, configuration, templating). Published to
  crates.io.
- `app/` — `obsidian-logging-cli`, the CLI binary. Depends on `lib/`. Distributed as pre-built
  binaries via GitHub Releases.

## Using the justfile

The project includes a `justfile` for common tasks. Install [`just`](https://github.com/casey/just)
if you don't already have it:

```bash
cargo install just
```

Then:

```bash
# Building
just build              # Build lib + app
just build-lib           # Build library only
just build-app           # Build CLI only
just build-release       # Build in release mode

# Testing
just test                # Test everything
just test-lib            # Test library only
just test-app            # Test CLI only

# Code quality
just fmt                 # Format all code
just fmt-check           # Check formatting without modifying
just clippy              # Run clippy linter
just check               # fmt-check + clippy + test (recommended before committing)

# Running and debugging
just run --help          # Run CLI with arguments
just version             # Show version

# Release workflow
just release                 # Release with auto-incremented patch version
just release-version 1.4.0   # Release a specific version
just publish                 # Publish the library to crates.io

# Utility
just help                # List all available commands
just clean                # Clean build artifacts
```

Run `just check` before committing — it runs formatting checks, clippy (with warnings as errors),
and the full test suite.

## Running tests directly with Cargo

```bash
cargo test --workspace
cargo test -p obsidian-logging        # library tests only
cargo test -p obsidian-logging-cli    # CLI tests only
```

Library tests live under `lib/tests/` (one file per area: `add_tests.rs`, `config_tests.rs`,
`edit_tests.rs`, `list_tests.rs`, `template_tests.rs`, `utils_tests.rs`).

## Building this manual

This user manual is an [mdBook](https://rust-lang.github.io/mdBook/) project under `docs/`. To build
or preview it locally:

```bash
cd docs
mdbook build     # outputs static HTML to docs/book/
mdbook serve     # serves the book locally with live reload
```
