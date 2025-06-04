mod config;
mod utils;
mod commands;
mod template;

use crate::commands::{list, edit, remove, add};
use std::env;
use std::str::FromStr;
use config::{load_config, ListType};

fn print_help() {
    eprintln!("olog [-t hh:mm] <sentence> | -l | -b <days> | -u | -e | -T <list-type>");
    eprintln!("\nOptions:");
    eprintln!("  -t, --time hh:mm    Override timestamp for the entry");
    eprintln!("  -l, --list          List today's entries");
    eprintln!("  -b <days>           List entries from <days> days ago");
    eprintln!("  -u, --undo          Remove the last entry");
    eprintln!("  -e, --edit          Edit today's file");
    eprintln!("  -T <list-type>      Override list type (bullet or table)");
    std::process::exit(1);
}

fn main() {
    let mut config = load_config();
    let args = env::args().skip(1);
    
    // Process potential list type override first
    let args: Vec<String> = args.collect();
    let mut i = 0;
    let mut args_iter = args.iter().peekable();

    while let Some(arg) = args_iter.next() {
        if arg == "-T" {
            if let Some(list_type_str) = args_iter.next() {
                match ListType::from_str(list_type_str) {
                    Ok(list_type) => {
                        config = config.with_list_type(list_type);
                        i += 2;
                    }
                    Err(_) => {
                        eprintln!("Error: Invalid list type '{}'. Expected 'bullet' or 'table'", list_type_str);
                        std::process::exit(1);
                    }
                }
            } else {
                eprintln!("Error: -T requires a list type argument (bullet or table)");
                std::process::exit(1);
            }
        } else {
            break;
        }
    }

    // Process remaining arguments
    let remaining_args = args.into_iter().skip(i);
    let mut remaining_args = remaining_args.peekable();
    
    match remaining_args.next() {
        Some(arg) => match arg.as_str() {
            "-l" | "--list" => list::list_log_for_day(0, &config),
            "-b" => list::list_relative_day(&mut remaining_args, &config),
            "-u" => remove::remove_last_log_entry(&config),
            "-e" | "--edit" => edit::edit_today_log(&config),
            "-t" | "--time" => add::handle_with_time(remaining_args, &config),
            "-h" | "--help" => print_help(),
            other => add::handle_plain_entry(other.to_string(), remaining_args, &config),
        },
        None => {
            list::list_log_for_day(0, &config);
        }
    }
}

