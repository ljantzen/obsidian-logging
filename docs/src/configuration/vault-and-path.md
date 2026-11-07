# Vault and File Path

## `vault`

The absolute path to the root of your Obsidian vault:

```yaml
vault: /home/user/obsidian/
```

All daily note paths are resolved relative to this directory. It can be overridden at runtime with the
[`OBSIDIAN_VAULT_DIR`](./environment-variables.md) environment variable.

## `file_path_format`

A template describing where, inside the vault, today's daily note lives, and how it is named:

```yaml
file_path_format: "10-Journal/{year}/{month}/{date}.md"
```

Three placeholders are supported:

| Placeholder | Expands to                  | Example  |
|-------------|------------------------------|----------|
| `{year}`    | 4-digit year                 | `2024`   |
| `{month}`   | 2-digit month                 | `01`     |
| `{date}`    | Full ISO date (`YYYY-MM-DD`) | `2024-01-15` |

With the example above, an entry logged on 2024-01-15 would be written to
`/home/user/obsidian/10-Journal/2024/01/2024-01-15.md`.

If `file_path_format` is not set, it defaults to:

- Linux/macOS: `10-Journal/{year}/{month}/{date}.md`
- Windows: `10-Journal\{year}\{month}\{date}.md`

You're free to flatten this to a single directory (`file_path_format: "Journal/{date}.md"`) or nest it
however your vault is organized — `obsidian-logging` will create any directories it needs when it
writes the first entry for a given day.
