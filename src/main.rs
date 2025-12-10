use clap::{Parser, ValueEnum};
use obsidian_logging::{add, edit, list, Config, ListType, TimeFormat};
use std::env;
use std::io::{self, Read};

#[derive(Parser)]
#[command(
    name = "obsidian-logging",
    version = env!("CARGO_PKG_VERSION"),
    disable_version_flag = true,
    about = "A journaling/logging CLI that stores logs in Obsidian markdown files",
    long_about = "obsidian-logging is a command-line tool for creating and managing log entries in Obsidian markdown files. It supports various formats and can be configured through a YAML configuration file.

USAGE EXAMPLES:
  obsidian-logging                   # List today's entries
  obsidian-logging log entry         # Add a new log entry
  obsidian-logging -t 14:30 entry    # Add entry with specific time (seconds default to 00)
  obsidian-logging -t 14:30:45 entry # Add entry with specific time including seconds
  obsidian-logging -c work meeting   # Add entry to work category section
  obsidian-logging -c personal gym   # Add entry to personal category section
  obsidian-logging -p meeting        # Use predefined phrase from config
  obsidian-logging -p gym -c health  # Use phrase with category
  obsidian-logging -p meeting John   # Use phrase with argument expansion
  obsidian-logging -p call {0}       # Use phrase with placeholder {0}
  obsidian-logging -l                # List today's entries
  obsidian-logging -b 1              # List entries from 1 day ago
  obsidian-logging -e                # Edit today's file
  obsidian-logging -b 1 -e           # Edit file from 1 day ago
  obsidian-logging -T table -l       # List in table format
  obsidian-logging -f 12 -t 2:30 PM  # Use 12-hour format with time
  echo \"My log entry\" | obsidian-logging -S        # Read from stdin
  cat file.txt | obsidian-logging -S                 # Read from file via pipe

CONFIGURATION:
  Configuration file location:
    Linux/macOS: ~/.config/obsidian-logging/obsidian-logging.yaml
    Windows: %APPDATA%\\obsidian-logging\\obsidian-logging.yaml

  Environment variable: $OBSIDIAN_VAULT_DIR (overrides vault setting in config)

TEMPLATE VARIABLES:
  {today}      Current date (YYYY-MM-DD)
  {yesterday}  Yesterday's date
  {tomorrow}   Tomorrow's date
  {weekday}    Localized weekday name
  {created}    Creation timestamp (YYYY-MM-DD HH:mm:ss)"
)]
struct Cli {
    /// Override list type (bullet or table)
    #[arg(short = 'T', value_enum, help = "Override list type: bullet or table")]
    list_type: Option<ListTypeArg>,

    /// Override time format (12 or 24)
    #[arg(short = 'f', value_enum, help = "Override time format: 12 or 24")]
    time_format: Option<TimeFormatArg>,

    /// Override timestamp for the entry (format: hh:mm or hh:mm:ss, or hh:mm AM/PM or hh:mm:ss AM/PM)
    #[arg(
        short,
        long,
        help = "Override timestamp (e.g., 14:30, 14:30:45, 2:30 PM, or 2:30:45 PM). If seconds are not provided, defaults to 00."
    )]
    time: Option<String>,

    /// List entries from specified days ago
    #[arg(
        short = 'b',
        default_value = "0",
        help = "Days ago (0 = today, 1 = yesterday, etc.)"
    )]
    days_ago: i64,

    /// Edit today's file or file from specified days ago
    #[arg(short, long, help = "Open file in $EDITOR (defaults to vim)")]
    edit: bool,

    /// List today's entries
    #[arg(
        short,
        long,
        help = "List entries (default action when no entry provided)"
    )]
    list: bool,

    /// Suppress output
    #[arg(short, long, help = "Suppress output")]
    silent: bool,

    /// Read log entry from stdin
    #[arg(
        short = 'S',
        long,
        help = "Read log entry from stdin instead of command line arguments"
    )]
    stdin: bool,

    /// Include table header when listing entries
    #[arg(short = 'H', long, help = "Include table header when listing entries")]
    header: bool,

    /// Category for the log entry (uses section_header_<category> from config)
    /// Can be specified multiple times to list multiple categories
    /// Use 'all' to list all categories
    #[arg(
        short = 'c',
        long,
        help = "Category for the log entry (uses section_header_<category> from config). Can be specified multiple times. Use 'all' to list all categories."
    )]
    category: Vec<String>,

    /// Use a predefined phrase from config (shorthand reference)
    /// Supports argument expansion with placeholders: {0}, {1}, {2}, etc. for specific arguments, or {*} for all arguments
    #[arg(
        short = 'p',
        long,
        help = "Use a predefined phrase from config (shorthand reference). Supports argument expansion with placeholders: {0}, {1}, {2}, etc. for specific arguments, or {*} for all arguments"
    )]
    phrase: Option<String>,

    /// The log entry text to add
    #[arg(help = "Log entry text (if not provided, lists entries)")]
    entry: Vec<String>,

    /// Print version information
    #[arg(short = 'v', long, help = "Print version information")]
    version: bool,
}

#[derive(ValueEnum, Clone)]
enum ListTypeArg {
    Bullet,
    Table,
}

#[derive(ValueEnum, Clone)]
enum TimeFormatArg {
    #[value(name = "12")]
    Hour12,
    #[value(name = "24")]
    Hour24,
}

impl From<ListTypeArg> for ListType {
    fn from(arg: ListTypeArg) -> Self {
        match arg {
            ListTypeArg::Bullet => ListType::Bullet,
            ListTypeArg::Table => ListType::Table,
        }
    }
}

impl From<TimeFormatArg> for TimeFormat {
    fn from(arg: TimeFormatArg) -> Self {
        match arg {
            TimeFormatArg::Hour12 => TimeFormat::Hour12,
            TimeFormatArg::Hour24 => TimeFormat::Hour24,
        }
    }
}

/// Expands argument placeholders in a phrase with actual CLI arguments.
///
/// Supports placeholders like {0}, {1}, {2}, etc. where the number corresponds
/// to the argument index. Also supports {*} to insert all remaining arguments
/// and {#} to insert all arguments with comma separation and proper conjunction.
///
/// # Arguments
///
/// * `phrase` - The phrase template with placeholders
/// * `args` - The CLI arguments to substitute
/// * `config` - The configuration containing conjunction setting
///
/// # Returns
///
/// The expanded phrase with arguments substituted
fn expand_phrase_arguments(phrase: &str, args: &[String], config: &Config) -> String {
    let mut result = phrase.to_string();

    // Replace {#} with comma-separated list with proper conjunction
    if result.contains("{#}") {
        let formatted_args = if args.is_empty() {
            String::new()
        } else if args.len() == 1 {
            args[0].clone()
        } else if args.len() == 2 {
            format!("{} {} {}", args[0], config.get_conjunction(), args[1])
        } else {
            let mut formatted = String::new();
            for (i, arg) in args.iter().enumerate() {
                if i > 0 {
                    if i == args.len() - 1 {
                        formatted.push_str(&format!(" {} {}", config.get_conjunction(), arg));
                    } else {
                        formatted.push_str(&format!(", {}", arg));
                    }
                } else {
                    formatted.push_str(arg);
                }
            }
            formatted
        };
        result = result.replace("{#}", &formatted_args);
    }

    // Replace {*} with all arguments joined by spaces
    if result.contains("{*}") {
        let all_args = args.join(" ");
        result = result.replace("{*}", &all_args);
    }

    // Replace numbered placeholders {0}, {1}, {2}, etc.
    for (i, arg) in args.iter().enumerate() {
        let placeholder = format!("{{{}}}", i);
        result = result.replace(&placeholder, arg);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_phrase_arguments() {
        use obsidian_logging::config::Config;
        use std::collections::HashMap;

        let config = Config {
            vault: "".to_string(),
            file_path_format: "".to_string(),
            section_header: "".to_string(),
            list_type: obsidian_logging::config::ListType::Bullet,
            template_path: None,
            locale: None,
            time_format: obsidian_logging::config::TimeFormat::Hour24,
            time_label: "".to_string(),
            event_label: "".to_string(),
            category_headers: HashMap::new(),
            phrases: HashMap::new(),
        };

        // Test basic expansion
        let phrase = "Hello {0}";
        let args = vec!["World".to_string()];
        let result = expand_phrase_arguments(phrase, &args, &config);
        assert_eq!(result, "Hello World");

        // Test multiple arguments
        let phrase = "Meeting with {0} and {1}";
        let args = vec!["John".to_string(), "Jane".to_string()];
        let result = expand_phrase_arguments(phrase, &args, &config);
        assert_eq!(result, "Meeting with John and Jane");

        // Test {*} expansion
        let phrase = "All arguments: {*}";
        let args = vec!["arg1".to_string(), "arg2".to_string(), "arg3".to_string()];
        let result = expand_phrase_arguments(phrase, &args, &config);
        assert_eq!(result, "All arguments: arg1 arg2 arg3");

        // Test {#} expansion with two items
        let phrase = "Meeting with {#}";
        let args = vec!["John".to_string(), "Jane".to_string()];
        let result = expand_phrase_arguments(phrase, &args, &config);
        assert_eq!(result, "Meeting with John and Jane");

        // Test {#} expansion with three items
        let phrase = "Meeting with {#}";
        let args = vec!["John".to_string(), "Jane".to_string(), "Bob".to_string()];
        let result = expand_phrase_arguments(phrase, &args, &config);
        assert_eq!(result, "Meeting with John, Jane and Bob");

        // Test {#} expansion with one item
        let phrase = "Meeting with {#}";
        let args = vec!["John".to_string()];
        let result = expand_phrase_arguments(phrase, &args, &config);
        assert_eq!(result, "Meeting with John");

        // Test mixed placeholders
        let phrase = "First: {0}, All: {*}";
        let args = vec!["first".to_string(), "second".to_string()];
        let result = expand_phrase_arguments(phrase, &args, &config);
        assert_eq!(result, "First: first, All: first second");

        // Test no placeholders
        let phrase = "No placeholders here";
        let args = vec!["ignored".to_string()];
        let result = expand_phrase_arguments(phrase, &args, &config);
        assert_eq!(result, "No placeholders here");
    }
}

fn main() {
    let cli = Cli::parse();

    // Handle version flag
    if cli.version {
        println!("obsidian-logging {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    let mut config = Config::initialize();

    // Apply format overrides if specified
    if let Some(list_type) = cli.list_type {
        config = config.with_list_type(list_type.into());
    }

    if let Some(time_format) = cli.time_format {
        config = config.with_time_format(time_format.into());
    }

    // Handle phrase expansion if specified
    let entry_text = if let Some(phrase_key) = &cli.phrase {
        if let Some(phrase_value) = config.phrases.get(phrase_key) {
            // Expand arguments in the phrase
            expand_phrase_arguments(phrase_value, &cli.entry, &config)
        } else {
            eprintln!("Error: Phrase '{}' not found in configuration", phrase_key);
            std::process::exit(1);
        }
    } else if !cli.entry.is_empty() {
        cli.entry.join(" ")
    } else {
        String::new()
    };

    // Determine the command to execute
    if cli.edit {
        // Edit command
        edit::edit_log_for_day(cli.days_ago, &config, cli.silent);
    } else if cli.list {
        // List command
        list::list_log_for_day(cli.days_ago, &config, cli.silent, cli.header, &cli.category);
    } else if cli.stdin {
        // Read entry from stdin
        let mut stdin_content = String::new();
        if let Err(e) = io::stdin().read_to_string(&mut stdin_content) {
            eprintln!("Error reading from stdin: {}", e);
            std::process::exit(1);
        }

        let entry = stdin_content.trim();
        if entry.is_empty() {
            eprintln!("Error: No content read from stdin");
            std::process::exit(1);
        }

        // Split the entry into words for processing
        let entry_words: Vec<String> = entry.split_whitespace().map(|s| s.to_string()).collect();

        if let Some(time) = cli.time {
            // Handle with specific time - include all entry words
            let mut time_args = vec![time];
            time_args.extend(entry_words);
            add::handle_with_time(
                time_args.into_iter(),
                &config,
                cli.silent,
                cli.category.first().map(|s| s.as_str()),
            );
        } else {
            // Handle plain entry
            let mut args = entry_words.into_iter();
            if let Some(first) = args.next() {
                add::handle_plain_entry(
                    first,
                    args,
                    &config,
                    cli.silent,
                    cli.category.first().map(|s| s.as_str()),
                );
            }
        }
    } else if !entry_text.is_empty() {
        // Add entry command
        if let Some(time) = cli.time {
            // Handle with specific time - include all entry words
            let mut time_args = vec![time];
            time_args.extend(entry_text.split_whitespace().map(|s| s.to_string()));
            add::handle_with_time(
                time_args.into_iter(),
                &config,
                cli.silent,
                cli.category.first().map(|s| s.as_str()),
            );
        } else {
            // Handle plain entry
            let mut args = entry_text.split_whitespace().map(|s| s.to_string());
            if let Some(first) = args.next() {
                add::handle_plain_entry(
                    first,
                    args,
                    &config,
                    cli.silent,
                    cli.category.first().map(|s| s.as_str()),
                );
            }
        }
    } else {
        // Default: list today's entries
        list::list_log_for_day(cli.days_ago, &config, cli.silent, cli.header, &cli.category);
    }
}
