# Reading from stdin

## `-S` / `--stdin`

Reads the log entry text from standard input instead of from the command-line arguments. This is
useful for piping output from another command straight into today's note:

```bash
echo "Quick note" | obsidian-logging -S
cat notes.txt | obsidian-logging -c work -S
```

Carriage-return and line-feed characters are stripped, and the input is logged as a single line,
regardless of how many lines were piped in.

`-S` can be combined with the same flags as a normal add — `-t`/`--time`, `-c`/`--category`,
`-T`, `-f`, and `-s`:

```bash
echo "Standup notes" | obsidian-logging -S -c work -t 9:00
```
