# Locale

## `locale`

Sets the locale used for two things: localized weekday names in [templates](./templates.md), and the
conjunction word used when expanding the `{#}` placeholder in [phrases](./phrases.md).

```yaml
locale: en_US    # English
```

If `locale` is left out of the config entirely, weekday names fall back to English and the conjunction
falls back to `and`. If `locale` **is** set but to a value that isn't recognized, the behavior differs
between the two features — see the notes below each table.

### Weekday names (`{weekday}` template variable)

Recognized locale codes for weekday formatting (used by the `{weekday}` template placeholder):

| Locale code | Language          |
|-------------|--------------------|
| `en_US`     | English            |
| `nb_NO`     | Norwegian Bokmål   |
| `nn_NO`     | Norwegian Nynorsk  |
| `de_DE`     | German             |
| `fr_FR`     | French             |
| `es_ES`     | Spanish            |
| `it_IT`     | Italian            |
| `ja_JP`     | Japanese           |
| `ko_KR`     | Korean             |
| `ru_RU`     | Russian            |
| `zh_CN`     | Chinese            |

If `locale` is set but not in this table, weekday names currently fall back to **Norwegian Bokmål**
(not English) — only an entirely absent `locale` setting produces English weekday names.

### Conjunction (`{#}` phrase placeholder)

A shorter locale code controls the word used to join the last two items in a `{#}` expansion (see
[Predefined Phrases](./phrases.md)):

| Locale code(s)      | Conjunction |
|----------------------|-------------|
| `no`, `nb`, `nn`     | og          |
| `da`                 | og          |
| `sv`                 | och         |
| `de`                 | und         |
| `fr`                 | et          |
| `es`                 | y           |
| `it`                 | e           |
| `pt`                 | e           |
| `ru`                 | и           |
| `ja`                 | と          |
| `ko`                 | 와          |
| `zh`                 | 和          |
| anything else        | and         |

**Note:** the weekday table matches long locale codes (`de_DE`) and the conjunction table matches
short codes (`de`) exactly, so a single `locale` value generally only localizes one of the two:

- `locale: de_DE` → German weekday names, but the conjunction falls back to `and` (`"de_DE"` doesn't
  match the short code `"de"`).
- `locale: de` → the conjunction is correctly `und`, but the weekday name falls back to Norwegian
  Bokmål (`"de"` isn't in the weekday table, so it hits that table's wildcard case above).

There's currently no single value that localizes both for a given language other than Norwegian
(`no`/`nb`/`nn`), where the short form happens to also work for weekdays via the wildcard fallback.
