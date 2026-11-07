# Troubleshooting

## My entry didn't get added where I expected

- Check `vault` and `file_path_format` in your config — the computed path is
  `<vault>/<file_path_format with {year}/{month}/{date} substituted>`. See
  [Vault and File Path](./configuration/vault-and-path.md).
- If you used `-c <category>`, make sure `section_header_<category>` is actually defined in your
  config. If it isn't, the entry silently goes to the default section instead.
- `$OBSIDIAN_VAULT_DIR`, if set, always wins over the `vault` value in the config file — check your
  shell environment if entries are landing in an unexpected vault.

## My entries appear in the wrong section, or categories "bleed" into each other

Log entries within a section must be consecutive, with no blank lines between them. The log block for
a section runs from its heading up to the next `##` heading (or end of file). If you manually add
blank lines or other content inside a log section, `obsidian-logging` may stop treating subsequent
lines as part of that section.

## My existing timestamps changed from `HH:mm` to `HH:mm:ss`

This is expected. Since version 1.3.0, adding a new entry to a section reformats every existing
timestamp in that section to include seconds, if it doesn't already have them. See
[List Type and Time Format](./configuration/list-and-time-format.md).

## `-p <phrase>` fails with "Phrase not found"

The name after `-p` must exactly match a key under `phrases:` in your config file. Check spelling and
indentation in your YAML — phrase names are case-sensitive.

## Weekday names or `{#}` conjunctions aren't in the language I expected

The `locale` setting is matched slightly differently for weekday names (long codes, e.g. `de_DE`) and
for phrase conjunctions (short codes, e.g. `de`) — a single value generally only localizes one of the
two. See [Locale](./configuration/locale.md) for the full matching tables and a worked example.

## `-e` opens the wrong editor, or nothing happens

`-e` uses `$EDITOR`. If it's unset, `vim` is used. Set `$EDITOR` in your shell profile if you'd
rather use something else:

```bash
export EDITOR=nano
```

## Nothing is printed at all

Check whether you (or a script calling `obsidian-logging`) passed `-s`/`--silent` — it suppresses
all output, including error messages, by design.

## Still stuck?

Open an issue on [GitHub](https://github.com/ljantzen/obsidian-logging/issues) with your config file
(with any personal paths redacted) and the exact command you ran.
