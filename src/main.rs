mod config;
mod utils;
mod commands;
mod template;

use crate::commands::{list, edit, remove, add};
use std::env;
use std::str::FromStr;
use config::{ListType, TimeFormat, initialize_config};

fn print_help() {
    eprintln!("obsidian-logging [options] [log entry]");
    eprintln!("\nUsage:");
    eprintln!("  obsidian-logging             List today's entries");
    eprintln!("  obsidian-logging <log entry> Add a new log entry");
    eprintln!("\nOptions:");
    eprintln!("  -t, --time hh:mm         Override timestamp for the entry");
    eprintln!("  -l, --list               List today's entries");
    eprintln!("  -b <days>                List entries from <days> days ago");
    eprintln!("  -u, --undo               Remove the last entry");
    eprintln!("  -e, --edit               Edit today's file");
    eprintln!("  -T <list-type>           Override list type (bullet or table)");
    eprintln!("  -f <time-format>         Override time format (12 or 24)");
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

    // First pass: process only format flags (-T and -f)
    while i < args.len() {
        match args[i].as_str() {
            "-T" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("Error: -T needs an argument (bullet or table)");
                    std::process::exit(1);
                }
                let list_type = ListType::from_str(&args[i]).unwrap_or_else(|_| {
                    eprintln!("Error: invalid list type '{}'. Use 'bullet' or 'table'", args[i]);
                    std::process::exit(1);
                });
                config = config.with_list_type(list_type);
                i += 1;
            },
            "-f" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("Error: -f needs an argument (12 or 24)");
                    std::process::exit(1);
                }
                let time_format = TimeFormat::from_str(&args[i]).unwrap_or_else(|_| {
                    eprintln!("Error: invalid time format '{}'. Use '12' or '24'", args[i]);
                    std::process::exit(1);
                });
                config = config.with_time_format(time_format);
                i += 1;
            },
            _ => {
                i += 1;
            }
        }
    }

    // Second pass: process command flags
    i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-T" | "-f" => {
                // Skip format flags and their arguments as they were already processed
                i += 2;
            },
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
            "-u" => {
                command = Some("undo");
                i += 1;
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
            other => {
                command = Some("add");
                command_args.push(other.to_string());
                i += 1;
                while i < args.len() {
                    command_args.push(args[i].clone());
                    i += 1;
                }
                break;
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
        Some("undo") => remove::remove_last_log_entry(&config),
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::path::PathBuf;
    use tempfile::TempDir;
    use crate::config::Config;

    fn setup_test_env() -> (PathBuf, Config) {
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().to_path_buf();
        
        // Set up environment variables
        if cfg!(windows) {
            env::set_var("APPDATA", config_dir.to_str().unwrap().to_owned());
        } else {
            env::set_var("HOME", config_dir.to_str().unwrap().to_owned());
        }
        env::remove_var("OBSIDIAN_VAULT");

        // Create a basic config
        let config = Config {
            vault: "/test/vault".to_string(),
            file_path_format: "test/{year}/{month}/{date}.md".to_string(),
            section_header: "## Test".to_string(),
            list_type: ListType::Bullet,
            template_path: None,
            locale: None,
            time_format: TimeFormat::Hour24,
        };

        (config_dir, config)
    }

    #[test]
    fn test_time_format_flag() {
        let (_config_dir, mut config) = setup_test_env();

        // Test 12-hour format
        let args = vec![String::from("-f"), String::from("12")];
        let mut args = args.into_iter().peekable();
        if let Some(arg) = args.next() {
            if arg == "-f" {
                let time_format = args.next().unwrap();
                let time_format = TimeFormat::from_str(&time_format).unwrap();
                config = config.with_time_format(time_format);
            }
        }
        assert_eq!(config.time_format, TimeFormat::Hour12);

        // Test 24-hour format
        let args = vec![String::from("-f"), String::from("24")];
        let mut args = args.into_iter().peekable();
        if let Some(arg) = args.next() {
            if arg == "-f" {
                let time_format = args.next().unwrap();
                let time_format = TimeFormat::from_str(&time_format).unwrap();
                config = config.with_time_format(time_format);
            }
        }
        assert_eq!(config.time_format, TimeFormat::Hour24);

        // Test invalid format
        let args = vec![String::from("-f"), String::from("invalid")];
        let mut args = args.into_iter().peekable();
        if let Some(arg) = args.next() {
            if arg == "-f" {
                let time_format = args.next().unwrap();
                assert!(TimeFormat::from_str(&time_format).is_err());
            }
        }
    }

    #[test]
    fn test_time_format_with_back_flag() {
        let (_config_dir, mut config) = setup_test_env();

        // Test that -f flag is processed before -b flag
        let args = vec![
            String::from("-b"), String::from("4"),
            String::from("-f"), String::from("12"),
        ];
        let mut i = 0;
        let mut command = None;
        let mut command_args = Vec::new();

        // First pass: process only format flags
        while i < args.len() {
            match args[i].as_str() {
                "-f" => {
                    i += 1;
                    let time_format = TimeFormat::from_str(&args[i]).unwrap();
                    config = config.with_time_format(time_format);
                    i += 1;
                },
                _ => {
                    i += 1;
                }
            }
        }

        // Second pass: process command flags
        i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "-f" => {
                    i += 2;
                },
                "-b" => {
                    command = Some("back");
                    i += 1;
                    if i < args.len() {
                        command_args.push(args[i].clone());
                    }
                    i += 1;
                },
                _ => {
                    i += 1;
                }
            }
        }

        assert_eq!(config.time_format, TimeFormat::Hour12);
        assert_eq!(command, Some("back"));
        assert_eq!(command_args, vec!["4"]);
    }
}

