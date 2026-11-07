# Obsidian logging - Log to Obsidian Journal from command line 

[![Crates.io](https://img.shields.io/crates/v/obsidian-logging.svg)](https://crates.io/crates/obsidian-logging)
[![Documentation](https://docs.rs/obsidian-logging/badge.svg)](https://docs.rs/obsidian-logging)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

This little utility written in Rust makes it possible to log directly to todays daily note from the linux command line. 

For a full walkthrough of installation, configuration, and every command-line flag, see the
[mdBook user manual](https://ljantzen.github.io/obsidian-logging/). The source lives in `docs/`
and can be built locally with `cd docs && mdbook build`.

## Version 1.3.0 Changes - Timestamp Format Update

**Important:** Starting with version 1.3.0, timestamps now include seconds (HH:mm:ss format instead of HH:mm).

**What this means for you:**
- New entries will have timestamps with seconds (e.g., `14:30:45` instead of `14:30`)
- When using the `-t` flag, you can specify seconds: `-t 14:30:45` or omit them: `-t 14:30` (defaults to `:00`)
- **Existing log files:** When you add a new entry to a file, all existing timestamps in that log section will be automatically reformatted to include seconds (e.g., `14:30` becomes `14:30:00`)
- Duplicate timestamp detection: If you add an entry with a timestamp that already exists, the seconds will be incremented until a unique timestamp is found

This change ensures better precision and prevents timestamp collisions when logging multiple entries at the same minute.

## License 

This software is licensed under a combined MIT and SPPL license.  It is basically a MIT license, but in order to be compliant you need to send me a postcard.  Details in LICENSE.md

## Installation

### Binary (Command-line tool)

You can install the obsidian-logging CLI tool using cargo:

```bash
cargo install obsidian-logging-cli
```

Or download pre-built binaries directly from the [github releases page](https://github.com/ljantzen/obsidian-logging/releases).

### Library (For Rust projects)

To use obsidian-logging as a library in your Rust project:

```toml
[dependencies]
obsidian-logging = "1.3"
```

Then in your code:

```rust
use obsidian_logging::{add, edit, list, Config};
```

See the [API documentation](https://docs.rs/obsidian-logging) for library usage.

### Build from source

Clone the repository and build the CLI:

```bash
git clone https://github.com/ljantzen/obsidian-logging
cd obsidian-logging
cargo build --release -p obsidian-logging-cli
./target/release/obsidian-logging --version
```

Or build everything (library + CLI):

```bash
cargo build --workspace --release
```

## Project Structure

This project is organized as a Cargo workspace with two crates:

- **`lib/`** — The core library crate (`obsidian-logging`)
  - Contains all the logging, configuration, and file management logic
  - Published to [crates.io](https://crates.io/crates/obsidian-logging)
  - Can be used as a dependency in other Rust projects

- **`app/`** — The CLI binary crate (`obsidian-logging-cli`)
  - Provides the command-line interface and argument parsing
  - Depends on the library crate
  - Distributed as pre-built binaries via [GitHub Releases](https://github.com/ljantzen/obsidian-logging/releases)

This separation allows the logging functionality to be reused in other Rust applications while keeping the CLI focused and maintainable.

## Development

### Quick start

Clone and build the project:

```bash
git clone https://github.com/ljantzen/obsidian-logging
cd obsidian-logging
just build      # Build everything
just test       # Run all tests
just check      # Run fmt, clippy, and tests
```

### Using justfile

This project includes a `justfile` for common development tasks. Install `just` if you don't have it:

```bash
cargo install just
```

Then use `just` to run tasks:

```bash
# Building
just build              # Build lib + app
just build-lib          # Build library only
just build-app          # Build CLI only
just build-release      # Build in release mode

# Testing
just test               # Test everything
just test-lib           # Test library only
just test-app           # Test CLI only

# Code quality
just fmt                # Format all code
just fmt-check          # Check formatting without modifying
just clippy             # Run clippy linter
just check              # Run fmt-check + clippy + test (pre-commit)

# Running and debugging
just run --help         # Run CLI with arguments
just version            # Show version

# Release workflow
just release            # Release with auto-incremented patch version
just release-version 1.4.0  # Release specific version
just publish            # Publish library to crates.io

# Utility
just help               # List all available commands
just clean              # Clean build artifacts
```

The `check` target is recommended before committing:

```bash
just check
```

## Obsidian Native CLI

Obsidian now has a native CLI tool (available as Early Access with Catalyst License). You can see the [Obsidian CLI documentation](https://help.obsidian.md/cli) for more information.

### When to use obsidian-logging vs Obsidian CLI

**Use obsidian-logging if you want:**
- Quick, focused daily logging with timestamps
- Organized logs by category (work, personal, health, etc.)
- Reusable phrase templates for common entries
- A lightweight tool optimized for quick note capture
- Free and open-source solution

**Use Obsidian CLI if you need:**
- General vault operations and querying
- Advanced automation and scripting
- Plugin management
- Full programmatic control of Obsidian
- Integration with complex workflows

For simple daily journaling from the command line, obsidian-logging provides a streamlined, dedicated solution. For comprehensive vault automation, Obsidian's native CLI offers broader capabilities.

## Tip

Since obsidian-logging is quite a mouthful to type every time, it is recommended to create a short alias.  E.g

```
alias q=` obsidian-logging`
```
(Note: the space before the program name stops the command from being entered into command history)

On Windows, the `doskey` command can be used to create a macro: 

```
doskey q=obsidian-logging
```

## Configuration file

> See the [Configuration chapter](https://ljantzen.github.io/obsidian-logging/configuration.html) of the user manual for the complete reference, including [categories](https://ljantzen.github.io/obsidian-logging/configuration/categories.html), [phrases](https://ljantzen.github.io/obsidian-logging/configuration/phrases.html), [templates](https://ljantzen.github.io/obsidian-logging/configuration/templates.html), and [environment variables](https://ljantzen.github.io/obsidian-logging/configuration/environment-variables.html).

Obsidian-logging reads `~/.config/obsidian-logging/obsidian-logging.yaml` on startup, creating it and prompting for initial values if it doesn't exist. See `obsidian-logging.example.yaml` for a sample.

In short: entries are appended under a configurable `section_header` marker, categories (`-c`) can route entries to their own `section_header_<category>` section, and reusable phrase templates (`-p`) with `{0}`, `{*}`, `{#}` argument expansion can be defined under `phrases:`. The manual has the full details and examples.

## Command line switches 

> See the [Command-Line Reference](https://ljantzen.github.io/obsidian-logging/cli-reference.html) in the user manual for the complete, up-to-date flag reference, including chapters on [adding](https://ljantzen.github.io/obsidian-logging/cli/adding-entries.html), [listing](https://ljantzen.github.io/obsidian-logging/cli/listing-entries.html), and [editing](https://ljantzen.github.io/obsidian-logging/cli/editing-entries.html) entries, and [reading from stdin](https://ljantzen.github.io/obsidian-logging/cli/stdin.html).

Invoking `obsidian-logging` with no arguments lists today's entries (equivalent to `-l`). Invoking it with a plain string, e.g. `obsidian-logging "Had lunch"`, appends that string as a new timestamped entry to the default section.

## Usage Examples

```bash
obsidian-logging                              # List today's entries
obsidian-logging "Had lunch with colleagues"  # Add an entry with the current time
obsidian-logging -t 14:30 "Team standup"      # Add an entry with a specific time
obsidian-logging -c work "Code review"        # Add an entry to the "work" category
obsidian-logging -p meeting                   # Add an entry from a predefined phrase
echo "Quick note" | obsidian-logging -S       # Add an entry read from stdin
obsidian-logging -b 1                         # List yesterday's entries
obsidian-logging -l -c work                   # List entries from the "work" category
obsidian-logging -e                           # Edit today's file directly
```

See the [Command-Line Reference](https://ljantzen.github.io/obsidian-logging/cli-reference.html) for every flag and more examples.

## Example Output

With the category functionality, your daily notes can be organized into different sections. Here's an example of what a daily note might look like:

```markdown
# 2024-01-15

## 🕗

* 08:30:00 Morning coffee
* 12:00:00 Lunch break

## 💼 Work

* 09:00:00 Daily standup meeting
* 10:30:00 Code review completed
* 14:00:00 Client presentation

## 🏠 Personal

* 18:00:00 Gym workout - 45 minutes
* 19:30:00 Dinner with family

## 🏥 Health

* 11:00:00 Doctor appointment
* 16:00:00 Picked up prescription
```

**Note:** All timestamps now include seconds. If you have existing log files with timestamps in `HH:mm` format, they will be automatically reformatted to `HH:mm:ss` format (with `:00` seconds) when you add a new entry to that file.

## Screenshots

### Bullet mode 

![image](https://github.com/user-attachments/assets/72c50c59-5185-4cb4-a871-473a8fd8b96f)

### Table mode 

![image](https://github.com/user-attachments/assets/ad3fe2c4-9a33-4272-a059-3d22617cef97)


# Contact info 

https://mas.to/@jantzten

https://bsky.app/profile/leif.jantzen.no
