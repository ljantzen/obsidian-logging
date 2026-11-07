# Introduction

**obsidian-logging** is a small command-line tool, written in Rust, for logging directly to today's
daily note from the Linux (or macOS/Windows) command line. It is built for people who keep a daily
journal in [Obsidian](https://obsidian.md) and want a fast, keyboard-driven way to append timestamped
entries without opening the app.

Typical use:

```bash
obsidian-logging "Had lunch with colleagues"
```

This appends a timestamped line to today's daily note, under a configurable section heading. You can
also:

- organize entries into categories such as `work`, `personal`, or `health`, each with its own section
  in the note
- define shorthand **phrases** for common entries ("gym", "standup", ...) with optional argument
  expansion
- list today's (or a previous day's) entries back out on the command line
- open today's note directly in `$EDITOR`
- choose between bullet-list and markdown-table output, and between 12- and 24-hour timestamps

## Who this manual is for

This manual covers everyday usage of the `obsidian-logging` CLI: installing it, configuring your vault
and daily-note layout, and using every command-line switch. It also includes a short chapter on using
the underlying Rust library (`obsidian-logging` on crates.io) in your own projects, and a chapter for
contributors who want to build the project from source.

## Project layout

The project is a Cargo workspace with two crates:

- `obsidian-logging` (in `lib/`) — the core library: configuration, templating, and log file
  management logic. Published on [crates.io](https://crates.io/crates/obsidian-logging) and
  documented on [docs.rs](https://docs.rs/obsidian-logging).
- `obsidian-logging-cli` (in `app/`) — the command-line binary (`obsidian-logging`) that most users
  install. Distributed as pre-built binaries via
  [GitHub Releases](https://github.com/ljantzen/obsidian-logging/releases).

## Source and license

The source code is hosted at
[github.com/ljantzen/obsidian-logging](https://github.com/ljantzen/obsidian-logging). The project is
licensed under a combined MIT/SPPL license — see [License and Contact](./license.md) for details.
