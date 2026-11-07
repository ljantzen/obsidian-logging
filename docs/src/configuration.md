# Configuration

`obsidian-logging` reads its configuration from a YAML file:

- Linux/macOS: `~/.config/obsidian-logging/obsidian-logging.yaml`
- Windows: `%APPDATA%\obsidian-logging\obsidian-logging.yaml`

If the file doesn't exist on first run, `obsidian-logging` creates it and prompts for some values. A
fully annotated example is shipped in the repository as
[`obsidian-logging.example.yaml`](https://github.com/ljantzen/obsidian-logging/blob/main/obsidian-logging.example.yaml).

The following pages describe each setting in detail:

- [Vault and File Path](./configuration/vault-and-path.md) — where your vault lives and how daily
  note filenames are built.
- [Section Headers and Categories](./configuration/categories.md) — the marker that delimits the log
  block, and per-category sections.
- [List Type and Time Format](./configuration/list-and-time-format.md) — bullet vs. table output,
  12-hour vs. 24-hour timestamps.
- [Locale](./configuration/locale.md) — localized weekday names and list conjunctions.
- [Templates](./configuration/templates.md) — the template used to seed a new daily note.
- [Predefined Phrases](./configuration/phrases.md) — shorthand log entries with argument expansion.
- [Environment Variables](./configuration/environment-variables.md) — overriding the vault path at
  runtime.

## Full example

```yaml
{{#include ../../obsidian-logging.example.yaml}}
```
