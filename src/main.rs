mod config;
mod utils;
mod commands;
mod template;

use std::env;
use std::str::FromStr;
use config::{initialize_config, ListType, TimeFormat};
use commands::{add, edit, list};

fn print_help() {
    eprintln!("obsidian-logging [options] [log entry]");
    eprintln!("\nUsage:");
    eprintln!("  obsidian-logging             List today's entries");
    eprintln!("  obsidian-logging <log entry> Add a new log entry");
    eprintln!();
    eprintln!("\nOptions:");
    eprintln!();
    eprintln!("-T and/or -f must come before any other options");
    eprintln!();
    eprintln!("  -T <list-type>           Override list type (bullet or table)");
    eprintln!("  -f <time-format>         Override time format (12 or 24)");
    eprintln!();
    eprintln!("  -t, --time hh:mm         Override timestamp for the entry");
    eprintln!("  -l, --list               List today's entries");
    eprintln!("  -b <days>                List entries from <days> days ago");
    eprintln!("  -e, --edit               Edit today's file");
    eprintln!("  -h, --help               Show this help message");
    eprintln!("  -v, --version            Show version information");
    eprintln!("\nConfiguration:");
    if cfg!(windows) {
        eprintln!("  Location: %APPDATA%\\obsidian-logging\\obsidian-logging.yaml");
    } else {
        eprintln!("  Location: ~/.config/obsidian-logging/obsidian-logging.yaml");
    }
    eprintln!("  vault:            Path to Obsidian vault, overrides $OBSIDIAN_VAULT_DIR");
    eprintln!("  file_path_format: Format for daily note directory path");
    eprintln!("  section_header:   Marker for log entries section");
    eprintln!("  list_type:        Default list format (bullet or table)");
    eprintln!("  time_format:      Default time format (12 or 24)");
    eprintln!("  template_path:    Path to template file");
    eprintln!("  locale:           Locale for weekday names (e.g., en_US, nb_NO)");
    eprintln!("\nTemplate Variables:");
    eprintln!("  {{today}}          Current date (YYYY-MM-DD)");
    eprintln!("  {{yesterday}}      Yesterday's date");
    eprintln!("  {{tomorrow}}       Tomorrow's date");
    eprintln!("  {{weekday}}        Localized weekday name");
    eprintln!("  {{created}}        Creation timestamp (YYYY-MM-DD HH:mm)");
    std::process::exit(1);
}

fn main() {
    let mut config = initialize_config();
    let args: Vec<String> = env::args().collect();
    let mut i = 1;
    let mut command = None;
    let mut command_args = Vec::new();
    let mut format_flags_processed = false;

    // Single pass for all arguments
    while i < args.len() {
        let arg = &args[i];

        // Process format flags only once at the beginning
        if !format_flags_processed {
            match arg.as_str() {
                "-T" => {
                    i += 1;
                    if i < args.len() {
                        let list_type = ListType::from_str(&args[i]).unwrap_or_else(|_| {
                            eprintln!("Error: invalid list type '{}'. Use 'bullet' or 'table'", args[i]);
                            std::process::exit(1);
                        });
                        config = config.with_list_type(list_type);
                        i += 1;
                        continue;
                    } else {
                        eprintln!("Error: -T needs an argument (bullet or table)");
                        std::process::exit(1);
                    }
                },
                "-f" => {
                    i += 1;
                    if i < args.len() {
                        let time_format = TimeFormat::from_str(&args[i]).unwrap_or_else(|_| {
                            eprintln!("Error: invalid time format '{}'. Use '12' or '24'", args[i]);
                            std::process::exit(1);
                        });
                        config = config.with_time_format(time_format);
                        i += 1;
                        continue;
                    } else {
                        eprintln!("Error: -f needs an argument (12 or 24)");
                        std::process::exit(1);
                    }
                },
                _ => {
                    format_flags_processed = true;
                }
            }
        }

        // After format flags, process the command
        match arg.as_str() {
            "-l" | "--list" => {
                command = Some("list");
                i += 1;
            },
            "-b" => {
                command = Some("back");
                i += 1;
                if i < args.len() {
                    command_args.push(args[i].clone());
                    i += 1;
                }
            },
            "-e" | "--edit" => {
                command = Some("edit");
                i += 1;
            },
            "-t" | "--time" => {
                command = Some("time");
                i += 1;
                while i < args.len() {
                    command_args.push(args[i].clone());
                    i += 1;
                }
                break;
            },
            "-h" | "--help" => {
                print_help();
                std::process::exit(0);
            },
            "-v" | "--version" => {
                println!("obsidian-logging version {}", env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            },
            // Ignore already processed format flags
            "-T" | "-f" => {
                i += 2;
            },
            other => {
                if command.is_none() {
                    command = Some("add");
                }
                command_args.push(other.to_string());
                i += 1;
                while i < args.len() {
                    // Stop collecting args if a flag is found
                    if args[i].starts_with('-') {
                        break;
                    }
                    command_args.push(args[i].clone());
                    i += 1;
                }
                if command == Some("add") {
                    break;
                }
            },
        }
    }

    // Execute the command with the processed config
    match command {
        Some("list") => list::list_log_for_day(0, &config),
        Some("back") => {
            let mut args = command_args.into_iter();
            list::list_relative_day(&mut args, &config)
        },
        Some("edit") => edit::edit_today_log(&config),
        Some("time") => {
            let args = command_args.into_iter();
            add::handle_with_time(args, &config)
        },
        Some("add") => {
            let mut args = command_args.into_iter();
            if let Some(first) = args.next() {
                add::handle_plain_entry(first, args, &config)
            }
        },
        None => list::list_log_for_day(0, &config),
        _ => unreachable!(),
    }
}

