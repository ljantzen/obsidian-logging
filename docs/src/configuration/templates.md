# Templates

When `obsidian-logging` needs to create a new daily note (because today's file doesn't exist yet), it
seeds the file with content from a template rather than starting from a blank file.

## `template_path`

```yaml
template_path: ~/.config/obsidian-logging/template.md
```

The path to a markdown template file. A leading `~` is expanded to your home directory. If
`template_path` is not set, or the file can't be read, the new note defaults to just:

```markdown
## 🕗

```

## Template variables

The template file can use the following placeholders, which are substituted when a new note is
created:

| Placeholder    | Expands to                                      | Example              |
|----------------|--------------------------------------------------|-----------------------|
| `{today}`      | Today's date (`YYYY-MM-DD`)                      | `2024-01-15`          |
| `{yesterday}`  | Yesterday's date (`YYYY-MM-DD`)                  | `2024-01-14`          |
| `{tomorrow}`   | Tomorrow's date (`YYYY-MM-DD`)                   | `2024-01-16`          |
| `{weekday}`    | Localized weekday name (see [Locale](./locale.md)) | `monday`           |
| `{created}`    | Creation timestamp (`YYYY-MM-DD HH:mm:ss`)       | `2024-01-15 08:12:03` |

## Example template

```markdown
{{#include ../../../template.example.md}}
```

With `locale: en_US` and a note created on Monday, 2024-01-15, this would expand to:

```markdown
# Daily Note for 2024-01-15

Previous: [[2024-01-14]]
Next: [[2024-01-16]]

Today is monday

## 🕗
```

Note that the template is responsible for including the `section_header` text itself (`## 🕗` in the
example above) — `obsidian-logging` doesn't add it separately. If you use
[category-specific sections](./categories.md), add those headings to the template too so they exist
before the first entry for that category is logged.
