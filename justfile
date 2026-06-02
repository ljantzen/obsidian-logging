#!/usr/bin/env just --justfile

set shell := ["zsh", "-cu"]

# Build lib crate
build-lib:
    cargo build -p obsidian-logging

# Build app crate
build-app:
    cargo build -p obsidian-logging-cli

# Build everything (lib + app)
build:
    cargo build --workspace

# Build all in release mode
build-release:
    cargo build --workspace --release

# Test lib crate
test-lib:
    cargo test -p obsidian-logging

# Test app crate
test-app:
    cargo test -p obsidian-logging-cli

# Test everything
test:
    cargo test --workspace

# Run clippy on lib
clippy-lib:
    cargo clippy -p obsidian-logging -- -D warnings

# Run clippy on app
clippy-app:
    cargo clippy -p obsidian-logging-cli -- -D warnings

# Run clippy on everything
clippy:
    cargo clippy --workspace -- -D warnings

# Format code (lib)
fmt-lib:
    cargo fmt -p obsidian-logging

# Format code (app)
fmt-app:
    cargo fmt -p obsidian-logging-cli

# Format all code
fmt:
    cargo fmt --all

# Check formatting without modifying
fmt-check:
    cargo fmt --all -- --check

# Run all checks (fmt, clippy, tests)
check: fmt-check clippy test

# Run all checks and build
all: check build-release

# Run the binary
run *ARGS:
    cargo run -p obsidian-logging-cli -- {{ ARGS }}

# Run with version flag
version:
    cargo run -p obsidian-logging-cli -- --version

# Clean build artifacts
clean:
    cargo clean

# Release with auto-incremented patch version
release:
    ./release.sh

# Release with specific version (e.g., just release-version 1.4.0)
release-version VERSION:
    ./release.sh {{ VERSION }}

# Publish lib crate to crates.io (must be run after GitHub release succeeds)
publish:
    cargo publish -p obsidian-logging --allow-dirty

# Publish with dry-run (validates without uploading)
publish-dry-run:
    cargo publish -p obsidian-logging --allow-dirty --dry-run

# Show help
help:
    @just --list
