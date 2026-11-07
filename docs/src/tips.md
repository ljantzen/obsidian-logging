# Tips and Tricks

## Create a short alias

Typing `obsidian-logging` for every quick note gets old fast. Add a short alias to your shell
config:

```bash
alias q=' obsidian-logging'
```

The leading space keeps the invocation itself out of your shell history in most shells. On Windows,
use `doskey`:

```text
doskey q=obsidian-logging
```

## Pipe command output straight into your journal

```bash
git log -1 --oneline | obsidian-logging -S -c work
uptime | obsidian-logging -S -c health
```

## Quickly check in without leaving the terminal

```bash
q -p standup -c work
q -p gym -c health
```

## Review the day

```bash
q -l -c all -T table -H
```

Lists every category for today as a markdown table with headers — a good end-of-day review command.

## Script a weekly summary

Because `-b` accepts any number of days, you can loop back over the past week in a shell script and
pull every day's entries for a given category, e.g. to build a weekly report:

```bash
for i in 0 1 2 3 4 5 6; do
  obsidian-logging -b "$i" -l -c work
done
```

## Keep the automatic seconds-migration in mind

Adding a new entry to a log section rewrites *every* existing timestamp in that section to include
seconds, if any are missing them. If you hand-edit timestamps in a daily note, adding a new entry
afterward will normalize them — this is expected and documented in
[List Type and Time Format](./configuration/list-and-time-format.md).
