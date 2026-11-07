# Using the Library

The CLI is a thin wrapper around the `obsidian-logging` library crate, which is published separately
on [crates.io](https://crates.io/crates/obsidian-logging) and documented on
[docs.rs](https://docs.rs/obsidian-logging). If you want to build your own tool on top of the same
logging engine — a GUI, a different CLI, a bot — you can depend on the library directly instead of
shelling out to the binary.

## Adding the dependency

```toml
[dependencies]
obsidian-logging = "1.3"
```

## What the library exposes

The crate root re-exports the pieces most consumers need:

```rust
use obsidian_logging::{add, edit, list, Config, ListType, TimeFormat};
```

- `Config` — the deserialized YAML configuration (vault path, section headers, categories, list type,
  time format, locale, phrases, ...). Load it with `Config::initialize()` to read
  `~/.config/obsidian-logging/obsidian-logging.yaml` (honoring `$OBSIDIAN_VAULT_DIR`) exactly as the
  CLI does, or construct one directly for a fully programmatic setup.
- `ListType` / `TimeFormat` — the enums backing `list_type` and `time_format` in the config, with
  `.with_list_type()` / `.with_time_format()` builder methods on `Config` for overriding them at
  runtime.
- `add` — functions for appending a new timestamped entry to a day's file (`handle_plain_entry`,
  `handle_with_time`), with category support.
- `list` — `list_log_for_day`, for retrieving/printing a day's entries, optionally filtered by
  category.
- `edit` — `edit_log_for_day`, for opening a day's file in `$EDITOR`.

The `config`, `template`, and `utils` modules are also public, exposing lower-level building blocks
such as `get_log_path_for_date`, `extract_log_entries`, and the template-variable expansion used for
new files.

## Full API documentation

For complete, up-to-date signatures and doc comments, see
[docs.rs/obsidian-logging](https://docs.rs/obsidian-logging). This manual focuses on the CLI; the
library's rustdoc is the source of truth for programmatic usage.
