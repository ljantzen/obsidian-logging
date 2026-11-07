# Command-Line Reference

```text
obsidian-logging [OPTIONS] [ENTRY]...
```

`obsidian-logging` has three modes of operation, chosen based on which flags and arguments you pass:

1. **Add an entry** — the default when you pass plain text and no `-l`/`-e`/`-S` flag.
2. **List entries** — triggered by `-l`, or by running with no entry text and no other action flag.
3. **Edit today's note** — triggered by `-e`.

The following pages cover each mode, plus a full table of every flag:

- [Adding Entries](./cli/adding-entries.md)
- [Listing Entries](./cli/listing-entries.md)
- [Editing Entries](./cli/editing-entries.md)
- [Reading from stdin](./cli/stdin.md)
- [Flag Reference](./cli/flag-reference.md)

## Quick examples

```bash
# List today's entries (no arguments)
obsidian-logging

# Add an entry to the default section
obsidian-logging "Had lunch with colleagues"

# Add an entry with a specific time
obsidian-logging -t 14:30 "Team standup meeting"

# Add an entry to a category
obsidian-logging -c work "Code review completed"

# Use a predefined phrase
obsidian-logging -p gym -c health

# Read an entry from stdin
echo "Quick note" | obsidian-logging -S

# List entries from a previous day
obsidian-logging -b 1

# List a specific category, in table format, with a header
obsidian-logging -l -c work -T table -H

# Edit today's file
obsidian-logging -e
```
