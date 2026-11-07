# Installation

## Pre-built binary

Pre-built binaries are published for each release on the
[GitHub releases page](https://github.com/ljantzen/obsidian-logging/releases). Download the archive
for your platform, extract it, and place the `obsidian-logging` binary somewhere on your `$PATH`.

## Install with Cargo

If you have a Rust toolchain installed, you can install the CLI directly from crates.io:

```bash
cargo install obsidian-logging-cli
```

This builds and installs the `obsidian-logging` binary into `~/.cargo/bin` (make sure that directory
is on your `$PATH`).

## Build from source

Clone the repository and build just the CLI:

```bash
git clone https://github.com/ljantzen/obsidian-logging
cd obsidian-logging
cargo build --release -p obsidian-logging-cli
./target/release/obsidian-logging --version
```

Or build the whole workspace (library + CLI):

```bash
cargo build --workspace --release
```

If you have [`just`](https://github.com/casey/just) installed, the project's `justfile` provides
shortcuts for common tasks — see [Development](./development.md).

## Verify the install

```bash
obsidian-logging -v
```

This prints the installed version and exits.

## Library use

If you want to use the logging engine as a library inside your own Rust project rather than the CLI,
see [Using the Library](./library.md).

## Create a shell alias

Since `obsidian-logging` is a mouthful to type every time, it's worth creating a short alias. For
example, on Linux/macOS:

```bash
alias q=' obsidian-logging'
```

The leading space before the command name prevents the alias invocation itself from being written to
your shell history (most shells skip history for commands that start with a space).

On Windows, `doskey` can be used to create an equivalent macro:

```text
doskey q=obsidian-logging
```
