# Listing Entries

## `-l` / `--list`

Lists the current day's log entries:

```bash
obsidian-logging -l
```

If `obsidian-logging` is invoked with **no arguments at all**, this is the default action — i.e.
`obsidian-logging` on its own is equivalent to `obsidian-logging -l`.

## Going back in time: `-b` / `--back`

```bash
obsidian-logging -b 1     # yesterday's entries
obsidian-logging -b 7     # one week ago
```

`-b 0` is equivalent to `-l` (today). `-b` can be combined with `-c` to filter by category, and with
`-e` to edit a past day's file instead of listing it.

## Filtering by category: `-c` / `--category`

```bash
obsidian-logging -l -c work                     # only the "work" section
obsidian-logging -l -c personal                  # only the "personal" section
obsidian-logging -l -c work -c personal          # "work" and "personal" sections
obsidian-logging -l -c all                       # every category, including the default section
obsidian-logging -l -c unknown                   # falls back to the default section
```

`-c` can be repeated to list multiple categories at once. Passing `all` lists every section in the
note, including the default. If a category name doesn't have a matching
`section_header_<category>` entry in the config, entries from the default section are shown instead.

## Output format: `-T` and `-H`

- `-T bullet` / `-T table` — override the configured `list_type` just for this listing.
- `-H` / `--header` — include the table header row (column labels + separator) when listing in table
  mode.

```bash
obsidian-logging -l -T table -H
```

## Examples

```bash
obsidian-logging                         # today, default section
obsidian-logging -b 1 -c work             # yesterday, work section only
obsidian-logging -l -c work -c personal   # today, work + personal sections
obsidian-logging -l -c all -T table -H    # today, all sections, as a table with header
```
