# Getting Started

## First run

The first time you run `obsidian-logging`, it looks for a configuration file at:

- Linux/macOS: `~/.config/obsidian-logging/obsidian-logging.yaml`
- Windows: `%APPDATA%\obsidian-logging\obsidian-logging.yaml`

If the file does not exist, `obsidian-logging` creates it and prompts you for some initial values
(most importantly, the path to your Obsidian vault). After that first run, you can edit the YAML file
directly — see [Configuration](./configuration.md) for everything that can be set.

## Adding your first entry

```bash
obsidian-logging "Had lunch with colleagues"
```

This appends a bullet-point entry with the current time to today's daily note, under the default
section header (`## 🕗` unless you've changed it).

## Create a short alias

Typing `obsidian-logging` every time gets old fast, so it's worth adding a short alias to your shell
config (e.g. `~/.bashrc` or `~/.zshrc`):

```bash
alias q=obsidian-logging
```

With this in place, logging an entry is as short as:

```bash
q "Had lunch with colleagues"
```

See [Tips and Tricks](./tips.md) for more on aliasing, including a history-skipping variant and the
Windows `doskey` equivalent.

## Listing today's entries

Running `obsidian-logging` with no arguments lists today's entries:

```bash
obsidian-logging
```

This is exactly equivalent to:

```bash
obsidian-logging -l
```

## Editing the note directly

```bash
obsidian-logging -e
```

Opens today's daily note in `$EDITOR` (defaults to `vim` if `$EDITOR` is unset).

## Where to go next

- [Configuration](./configuration.md) to set up your vault path, categories, templates and phrases.
- [Command-Line Reference](./cli-reference.md) for every flag in detail.
- [Example Daily Note](./example-output.md) to see what a fully configured note looks like.
