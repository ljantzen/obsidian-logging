# Flag Reference

| Flag | Long form | Argument | Description |
|------|-----------|----------|--------------|
| `-t` | `--time` | `HH:mm[:ss]` or `h:mm[:ss] AM/PM` | Override the timestamp for a new entry. Seconds default to `00` if omitted. See [Adding Entries](./adding-entries.md). |
| `-l` | `--list` | — | List entries instead of adding one. Default action when no entry text and no other action flag is given. See [Listing Entries](./listing-entries.md). |
| `-b` | — | `<days>` | List (or edit) entries from `<days>` days ago. `0` = today (default). See [Listing Entries](./listing-entries.md). |
| `-e` | `--edit` | — | Open today's (or `-b`-selected) file in `$EDITOR` (defaults to `vim`). See [Editing Entries](./editing-entries.md). |
| `-s` | `--silent` | — | Suppress all output, including error messages. |
| `-S` | `--stdin` | — | Read the log entry text from stdin instead of the command line. See [Reading from stdin](./stdin.md). |
| `-H` | `--header` | — | Include the table header row when listing in table format. |
| `-c` | `--category` | `<name>` | Target/filter by category. Repeatable. Use `all` with `-l` to list every category. See [Section Headers and Categories](../configuration/categories.md). |
| `-p` | `--phrase` | `<name>` | Use a predefined phrase from the config, with optional trailing arguments for placeholder expansion. See [Predefined Phrases](../configuration/phrases.md). |
| `-T` | — | `bullet` \| `table` | Override `list_type` for this invocation. |
| `-f` | — | `12` \| `24` | Override `time_format` for this invocation. |
| `-v` | `--version` | — | Print the version string and exit. |
| `-h` | `--help` | — | Print command-line help and exit. |
|  | *(positional)* | `<entry>...` | The log entry text. If omitted (and no `-l`/`-e`/`-S`), entries are listed instead. |

## Notes

- `-T` and `-f` take their value as a separate word (e.g. `-T table`, `-f 12`), not `-Ttable`.
- `-c`/`--category` may be given multiple times on a single command line; when adding an entry, only
  the **first** `-c` value is used as the target category, while when listing, **all** given values
  are used as filters.
- Running with absolutely no flags and no entry text (`obsidian-logging`) is shorthand for
  `obsidian-logging -l`.
- `-v`/`--version` and `-h`/`--help` short-circuit all other behavior — they print and exit
  immediately, regardless of any other flags present.

## Built-in help text

Running `obsidian-logging -h` prints usage, every flag, a list of usage examples, the configuration
file locations, the `$OBSIDIAN_VAULT_DIR` environment variable, and the full set of template
placeholders, all generated directly from the CLI's argument parser.
