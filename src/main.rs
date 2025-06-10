mod config;
mod utils;
mod commands;
mod template;

use std::env;
use std::str::FromStr;
use config::{initialize_config, Config, ListType, TimeFormat};
use commands::{add, edit, list, remove};

fn print_help() {
    println!("Usage: ol [COMMAND] [OPTIONS] [ARGUMENTS]");
    println!();
    println!("A simple CLI for logging notes to Obsidian.");
    println!();
    println!("Commands:");
    println!("  add [TEXT]             Add a new log entry (default).");
    println!("  list                   List all log entries for today.");
    println!("  back [DAYS]            List all log entries for a number of days back.");
    println!("  undo                   Remove the last log entry.");
    println!("  edit                   Open today's log file in the default editor.");
    println!();
    println!("Options:");
    println!("  -t, --time [TIME]      Add a log entry with a specific time (HH:mm or HH:mm am/pm).");
    println!("  -h, --help             Show this help message.");
    println!("  -v, --version          Show the version number.");
    println!("  -l, --list-type [TYPE] Override the list type (bullet or table).");
    println!("  -f, --time-format [FMT] Override the time format (12 or 24).");
    println!();
    println!("Configuration:");
    println!("  The configuration file is located at ~/.config/obsidian-logging/obsidian-logging.yaml.");
    println!("  You can set the following options:");
    println!("    vault: /path/to/your/obsidian/vault");
    println!("    file_path_format: \"path/to/your/daily/{{year}}/{{month}}/{{date}}.md\"");
    println!("    section_header: \"## Your Header\"");
    println!("    list_type: bullet # or table");
    println!("    template_path: /path/to/your/template.md");
    println!("    locale: en_US # for weekday names");
    println!("    time_format: 24 # or 12");
}

fn main() {
    let mut config = initialize_config();
    let args: Vec<String> = env::args().skip(1).collect();
    let mut command: Option<&str> = None;
    let mut command_args: Vec<String> = Vec::new();

    // First pass: process config overrides
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-l" | "--list-type" => {
                i += 1;
                if i < args.len() {
                    let list_type = ListType::from_str(&args[i]).unwrap();
                    config = config.with_list_type(list_type);
                    i += 1;
                }
            },
            "-f" | "--time-format" => {
                i += 1;
                if i < args.len() {
                    let time_format = TimeFormat::from_str(&args[i]).unwrap();
                    config = config.with_time_format(time_format);
                    i += 1;
                }
            },
            _ => i += 1,
        }
    }

    // Second pass: process commands and other arguments
    i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-l" | "--list-type" | "-f" | "--time-format" => {
                i += 2; // Skip flags and their arguments
            },
            "list" => {
                command = Some("list");
                i += 1;
            },
            "back" => {
                command = Some("back");
                i += 1;
                if i < args.len() {
                    command_args.push(args[i].clone());
                    i += 1;
                }
            },
            "undo" => {
                command = Some("undo");
                i += 1;
            },
            "edit" => {
                command = Some("edit");
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

