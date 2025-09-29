use clap::{Parser, ValueEnum};
use obsidian_logging::{Config, ListType, TimeFormat, add, edit, list};
use std::env;
use std::io::{self, Read};

#[derive(Parser)]
#[command(
    name = "obsidian-logging",
    version = env!("CARGO_PKG_VERSION"),
    about = "A journaling/logging CLI that stores logs in Obsidian markdown files",
    long_about = "obsidian-logging is a command-line tool for creating and managing log entries in Obsidian markdown files. It supports various formats and can be configured through a YAML configuration file.

USAGE EXAMPLES:
  obsidian-logging                   # List today's entries
  obsidian-logging log entry         # Add a new log entry
  obsidian-logging -t 14:30 entry    # Add entry with specific time
  obsidian-logging -c work meeting   # Add entry to work category section
  obsidian-logging -c personal gym   # Add entry to personal category section
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
    
    /// Suppress output
    #[arg(short, long, help = "Suppress output")]
    silent: bool,
    
    /// Read log entry from stdin
    #[arg(short = 'S', long, help = "Read log entry from stdin instead of command line arguments")]
    stdin: bool,
    
    /// Include table header when listing entries
    #[arg(short = 'H', long, help = "Include table header when listing entries")]
    header: bool,
    
    /// Category for the log entry (uses section_header_<category> from config)
    /// Can be specified multiple times to list multiple categories
    /// Use 'all' to list all categories
    #[arg(short = 'c', long, help = "Category for the log entry (uses section_header_<category> from config). Can be specified multiple times. Use 'all' to list all categories.")]
    category: Vec<String>,
    
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
    
    let mut config = Config::initialize();
    
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
            add::handle_with_time(time_args.into_iter(), &config, cli.silent, cli.category.first().map(|s| s.as_str()));
        } else {
            // Handle plain entry
            let mut args = entry_words.into_iter();
            if let Some(first) = args.next() {
                add::handle_plain_entry(first, args, &config, cli.silent, cli.category.first().map(|s| s.as_str()));
            }
        }
    } else if !cli.entry.is_empty() {
        // Add entry command
        if let Some(time) = cli.time {
            // Handle with specific time - include all entry words
            let mut time_args = vec![time];
            time_args.extend(cli.entry);
            add::handle_with_time(time_args.into_iter(), &config, cli.silent, cli.category.first().map(|s| s.as_str()));
        } else {
            // Handle plain entry
            let mut args = cli.entry.into_iter();
            if let Some(first) = args.next() {
                add::handle_plain_entry(first, args, &config, cli.silent, cli.category.first().map(|s| s.as_str()));
            }
        }
    } else {
        // Default: list today's entries
        list::list_log_for_day(cli.days_ago, &config, cli.silent, cli.header, &cli.category);
    }
}
