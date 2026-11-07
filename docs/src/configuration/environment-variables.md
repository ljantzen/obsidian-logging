# Environment Variables

## `OBSIDIAN_VAULT_DIR`

If set, this overrides the `vault` value from `obsidian-logging.yaml` for the current invocation:

```bash
OBSIDIAN_VAULT_DIR=/tmp/test-vault obsidian-logging "Testing a different vault"
```

This is useful for testing configuration changes, pointing at a secondary vault temporarily, or
running `obsidian-logging` inside scripts/CI without touching your personal config file.

## `$EDITOR`

Not specific to `obsidian-logging`'s config file, but used by the [`-e`/`--edit`](../cli/editing-entries.md)
flag to decide which editor to open today's note in. If `$EDITOR` is unset, `vim` is used.
