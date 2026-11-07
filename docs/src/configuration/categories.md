# Section Headers and Categories

## The default section marker

`obsidian-logging` looks for a specific markdown heading in your daily note that marks where the log
entries block begins. Log entries must be consecutive, with no blank lines between them — the first
blank line (or next heading) ends the block.

The default marker is configured with `section_header`:

```yaml
section_header: "## 🕗"
```

Everything from this heading until the next `##` heading (or the end of the file) is treated as the
log block for the default category.

## Category-specific section headers

Beyond the default section, `obsidian-logging` can log to any number of named **categories**, each
with its own section in the daily note. Define a category by adding a
`section_header_<category_name>` key:

```yaml
section_header: "## 🕗"
section_header_work: "## 💼 Work"
section_header_personal: "## 🏠 Personal"
section_header_health: "## 🏥 Health"
```

Use the `-c`/`--category` flag to target one of these sections:

```bash
obsidian-logging -c work "Meeting"
```

This appends the entry under the `## 💼 Work` heading instead of the default `## 🕗` section.

If a category passed with `-c` has no matching `section_header_<category>` entry in the config, the
entry falls back to the default section.

## Listing by category

The same `-c` flag filters what `-l`/`--list` prints:

```bash
obsidian-logging -l -c work             # only the "work" section
obsidian-logging -l -c work -c personal # "work" and "personal" sections
obsidian-logging -l -c all              # every category, including the default section
```

See [Listing Entries](../cli/listing-entries.md) for the full behavior.
