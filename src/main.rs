mod commands;
mod config;
mod template;
mod utils;

use clap::{Parser, ValueEnum};
use commands::{add, edit, list};
use config::{initialize_config, ListType, TimeFormat};

#[derive(Parser)]
#[command(
    name = "obsidian-logging",
    version = env!("CARGO_PKG_VERSION"),
    about = "A journaling/logging CLI that stores logs in Obsidian markdown files",
    long_about = "obsidian-logging is a command-line tool for creating and managing log entries in Obsidian markdown files. It supports various formats and can be configured through a YAML configuration file.

USAGE EXAMPLES:
  obsidian-logging                    # List today's entries
  obsidian-logging <log entry>       # Add a new log entry
  obsidian-logging -t 14:30 <entry>  # Add entry with specific time
  obsidian-logging -l                # List today's entries
  obsidian-logging -b 1              # List entries from 1 day ago
  obsidian-logging -e                # Edit today's file
  obsidian-logging -b 1 -e           # Edit file from 1 day ago
  obsidian-logging -T table -l       # List in table format
  obsidian-logging -f 12 -t 2:30 PM  # Use 12-hour format with time

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
  {created}    Creation timestamp (YYYY-MM-DD HH:mm)"
)]
struct Cli {
    /// Override list type (bullet or table)
    #[arg(short = 'T', value_enum, help = "Override list type: bullet or table")]
    list_type: Option<ListTypeArg>,
    
    /// Override time format (12 or 24)
    #[arg(short = 'f', value_enum, help = "Override time format: 12 or 24")]
    time_format: Option<TimeFormatArg>,
    
    /// Override timestamp for the entry (format: hh:mm or hh:mm AM/PM)
    #[arg(short, long, help = "Override timestamp (e.g., 14:30 or 2:30 PM)")]
    time: Option<String>,
    
    /// List entries from specified days ago
    #[arg(short = 'b', default_value = "0", help = "Days ago (0 = today, 1 = yesterday, etc.)")]
    days_ago: i64,
    
    /// Edit today's file or file from specified days ago
    #[arg(short, long, help = "Open file in $EDITOR (defaults to vim)")]
    edit: bool,
    
    /// List today's entries
    #[arg(short, long, help = "List entries (default action when no entry provided)")]
    list: bool,
    
    /// The log entry text to add
    #[arg(help = "Log entry text (if not provided, lists entries)")]
    entry: Vec<String>,
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

fn main() {
    let cli = Cli::parse();
    
    let mut config = initialize_config();
    
    // Apply format overrides if specified
    if let Some(list_type) = cli.list_type {
        config = config.with_list_type(list_type.into());
    }
    
    if let Some(time_format) = cli.time_format {
        config = config.with_time_format(time_format.into());
    }
    
    // Determine the command to execute
    if cli.edit {
        // Edit command
        edit::edit_log_for_day(cli.days_ago, &config);
    } else if cli.list {
        // List command
        list::list_log_for_day(cli.days_ago, &config);
    } else if !cli.entry.is_empty() {
        // Add entry command
        let mut args = cli.entry.into_iter();
        if let Some(first) = args.next() {
            if let Some(time) = cli.time {
                // Handle with specific time
                let mut time_args = vec![time];
                time_args.extend(args);
                add::handle_with_time(time_args.into_iter(), &config);
            } else {
                // Handle plain entry
                add::handle_plain_entry(first, args, &config);
            }
        }
    } else {
        // Default: list today's entries
        list::list_log_for_day(cli.days_ago, &config);
    }
}
