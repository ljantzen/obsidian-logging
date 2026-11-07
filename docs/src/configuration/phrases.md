# Predefined Phrases

You can define common log entries once in your configuration file and refer to them with a short
name via the `-p`/`--phrase` flag, instead of retyping the full text every time.

## Basic phrases

```yaml
phrases:
  meeting: "Team meeting with stakeholders"
  gym: "Workout at the gym"
  lunch: "Lunch break"
  doctor: "Doctor appointment"
```

```bash
obsidian-logging -p meeting
obsidian-logging -p gym -c health
```

## Phrases with argument expansion

A phrase can include placeholders that are filled in from any extra words you pass after `-p
<name>`:

| Placeholder | Expands to                                                      |
|-------------|------------------------------------------------------------------|
| `{0}`, `{1}`, `{2}`, ... | A specific argument by position (0-indexed)         |
| `{*}`       | All arguments, joined with spaces                                |
| `{#}`       | All arguments as a comma-separated list with a locale-aware conjunction before the last item |

```yaml
phrases:
  meeting_with: "Team meeting with {*}"     # all arguments
  call_with: "Phone call with {0}"          # first argument only
  project: "Working on {0}"                 # first argument only
  exercise: "Exercise: {*}"                 # all arguments
  travel: "Travel to {0}"                   # first argument only
  food: "Ate {*}"                           # all arguments

  # Using {#} for a natural-language list
  meeting_with_list: "Team meeting with {#}"
  call_with_list: "Phone call with {#}"
```

```bash
obsidian-logging -p meeting_with John Smith
# -> "Team meeting with John Smith"

obsidian-logging -p call_with Alice
# -> "Phone call with Alice"

obsidian-logging -p project "Project Alpha"
# -> "Working on Project Alpha"

obsidian-logging -p meeting_with_list John Jane
# -> "Team meeting with John and Jane"

obsidian-logging -p call_with_list Alice Bob Charlie
# -> "Phone call with Alice, Bob and Charlie"
```

The conjunction used by `{#}` (`and`, `og`, `und`, ...) is chosen based on the
[`locale`](./locale.md) setting.

Phrases can be combined with `-t`/`--time` and `-c`/`--category`:

```bash
obsidian-logging -p meeting_with John -t 14:30
obsidian-logging -p doctor_with "Dr. Smith" -c health
```

If a phrase name passed to `-p` doesn't exist in the config, `obsidian-logging` prints an error and
exits with a non-zero status instead of logging anything.

## A larger example

See
[`obsidian-logging.example.yaml`](https://github.com/ljantzen/obsidian-logging/blob/main/obsidian-logging.example.yaml)
in the repository for a complete set of example phrases covering work, personal, and health
categories.
