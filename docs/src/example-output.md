# Example Daily Note

With category sections configured (see
[Section Headers and Categories](./configuration/categories.md)), a daily note can look like this:

```markdown
# 2024-01-15

## 🕗

* 08:30:00 Morning coffee
* 12:00:00 Lunch break

## 💼 Work

* 09:00:00 Daily standup meeting
* 10:30:00 Code review completed
* 14:00:00 Client presentation

## 🏠 Personal

* 18:00:00 Gym workout - 45 minutes
* 19:30:00 Dinner with family

## 🏥 Health

* 11:00:00 Doctor appointment
* 16:00:00 Picked up prescription
```

This was produced with a configuration roughly like:

```yaml
section_header: "## 🕗"
section_header_work: "## 💼 Work"
section_header_personal: "## 🏠 Personal"
section_header_health: "## 🏥 Health"
list_type: bullet
```

and commands such as:

```bash
obsidian-logging "Morning coffee" -t 08:30
obsidian-logging -c work "Daily standup meeting" -t 09:00
obsidian-logging -c work "Code review completed" -t 10:30
obsidian-logging -c work "Client presentation" -t 14:00
obsidian-logging "Lunch break" -t 12:00
obsidian-logging -c personal "Gym workout - 45 minutes" -t 18:00
obsidian-logging -c personal "Dinner with family" -t 19:30
obsidian-logging -c health "Doctor appointment" -t 11:00
obsidian-logging -c health "Picked up prescription" -t 16:00
```

## Table mode

The same note, with `list_type: table` and `-H`/`--header` used when adding entries, renders each
section as a markdown table instead of a bullet list:

```markdown
## 🕗

| Tidspunkt | Hendelse       |
|-----------|-----------------|
| 08:30:00  | Morning coffee  |
| 12:00:00  | Lunch break     |
```

All timestamps shown include seconds (`HH:mm:ss`) — see
[List Type and Time Format](./configuration/list-and-time-format.md) for details on the automatic
seconds migration applied to older entries.
