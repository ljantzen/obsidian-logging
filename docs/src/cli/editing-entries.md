# Editing Entries

## `-e` / `--edit`

Opens today's daily note directly in your editor, instead of adding or listing an entry:

```bash
obsidian-logging -e
```

The editor used is `$EDITOR`, or `vim` if `$EDITOR` is not set.

Combine with `-b`/`--back` to edit a previous day's note:

```bash
obsidian-logging -b 1 -e      # edit yesterday's note
```

This is handy for correcting a typo, reordering entries manually, or adding freeform content around
the log section that the CLI itself doesn't manage.
