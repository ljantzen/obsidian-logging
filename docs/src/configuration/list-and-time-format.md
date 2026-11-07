# List Type and Time Format

## `list_type`

Controls how entries are rendered when appended or listed. Two values are supported:

```yaml
list_type: bullet   # or: table
```

- **`bullet`** — each entry is a markdown bullet with a leading timestamp:

  ```markdown
  - 14:30:45 Team standup meeting
  ```

- **`table`** — each entry is a row in a two-column markdown table:

  ```markdown
  | Tidspunkt | Hendelse              |
  |-----------|------------------------|
  | 14:30:45  | Team standup meeting   |
  ```

  (Column header text comes from `time_label`/`event_label`, see below.)

This can be overridden per-invocation with `-T bullet` or `-T table` — see
[Flag Reference](../cli/flag-reference.md).

If the existing log section in a file uses a different format than the one currently configured,
`obsidian-logging` automatically converts all existing entries in that section to the new format when
a new entry is added.

## `time_format`

Controls whether timestamps use a 12-hour or 24-hour clock:

```yaml
time_format: 24   # or: 12
```

- `24` → `14:30:45`
- `12` → `02:30:45 PM`

Defaults to `24` if not set. This can be overridden per-invocation with `-f 12` or `-f 24`.

> **Timestamps include seconds.** Since version 1.3.0, timestamps are always `HH:mm:ss`. When a new
> entry is added to a log section, any existing timestamps in that section that are missing seconds
> (`HH:mm`) are automatically rewritten to include `:00` seconds, so that all entries in the section
> stay consistently formatted. If a new entry's timestamp collides with an existing one, the seconds
> are incremented until a unique timestamp is found. Mixing 12-hour and 24-hour timestamps within the
> same log section may produce unpredictable sort order — pick one format and stick with it.

## `time_label` and `event_label`

When using `list_type: table`, these set the column header text:

```yaml
time_label: Tidspunkt   # defaults to "Tidspunkt" (Norwegian for "time")
event_label: Hendelse   # defaults to "Hendelse" (Norwegian for "event")
```

For an English table header, set:

```yaml
time_label: Time
event_label: Event
```

Headers are only written to the file when listing/adding with the `-H`/`--header` flag.
