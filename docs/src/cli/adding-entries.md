# Adding Entries

## Plain entries

Running `obsidian-logging` with text and no `-l`, `-e`, or `-S` flag appends that text as a new entry
to the default log section of today's daily note:

```bash
obsidian-logging This is a log entry
```

A timestamp for the current time is prepended, formatted according to the configured
[list type](../configuration/list-and-time-format.md):

- **Bullet** mode: `- 14:30:45 This is a log entry`
- **Table** mode: `| 14:30:45 | This is a log entry |`

Quoting the entry text is optional — all trailing positional arguments are joined with spaces — but
quoting is recommended if your entry contains flag-like tokens (e.g. starting with `-`).

## Overriding the time: `-t` / `--time`

```bash
obsidian-logging -t 14:30 "Team standup meeting"        # becomes 14:30:00
obsidian-logging -t 14:30:45 "Team standup meeting"      # explicit seconds
obsidian-logging -t "2:30 PM" "Afternoon meeting"        # becomes 02:30:00 PM
obsidian-logging -t "2:30:45 PM" "Afternoon meeting"     # explicit seconds
```

- Accepts `HH:mm`, `HH:mm:ss`, `h:mm AM/PM`, or `h:mm:ss AM/PM`.
- If seconds are omitted, they default to `00`.
- Entries are kept sorted chronologically within their section. If the given timestamp already
  exists in the section, the seconds are incremented until a unique timestamp is found.
- Adding a new entry reformats every existing timestamp in that section to include seconds, if any
  are missing them (see [List Type and Time Format](../configuration/list-and-time-format.md)).

## Targeting a category: `-c` / `--category`

```bash
obsidian-logging -c work "Team meeting at 2pm"
obsidian-logging -c personal "Gym workout"
obsidian-logging -c health "Doctor appointment"
```

Logs the entry to the section defined by `section_header_<category>` in your config, falling back to
the default section if no such header is configured. See
[Section Headers and Categories](../configuration/categories.md).

## Using a predefined phrase: `-p` / `--phrase`

```bash
obsidian-logging -p meeting
obsidian-logging -p meeting_with John Smith
obsidian-logging -p call_with Alice -t 14:30
```

See [Predefined Phrases](../configuration/phrases.md) for how to define phrases and use argument
placeholders (`{0}`, `{*}`, `{#}`).

## Overriding format just for this call

- `-T bullet` / `-T table` — override `list_type` for this invocation.
- `-f 12` / `-f 24` — override `time_format` for this invocation.

```bash
obsidian-logging -T table -t 9:00 "Daily standup"
obsidian-logging -f 12 -t 2:30 PM "Afternoon meeting"
```

## Silencing output: `-s` / `--silent`

Suppresses all output, including error messages — useful when calling `obsidian-logging` from
scripts or other automation where you don't want anything printed:

```bash
obsidian-logging -s "Logged quietly"
```
