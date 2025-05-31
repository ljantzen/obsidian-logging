mod config;
mod log;
mod utils;
mod commands;

use crate::commands::{list, edit, remove, add};
use std::env;
use config::load_config;

fn main() {
    let config = load_config();
    let mut args = env::args().skip(1);
    let first_arg = args.next();

    match first_arg.as_deref() {
        Some("-l") | Some("--list") => list::list_log_for_day(0, &config),
        Some("-b") => list::list_relative_day(&mut args, &config),
        Some("-n") => list::list_n_days_ago(&mut args, &config),
        Some("-u") => remove::remove_last_log_entry(&config),
        Some("-e") | Some("--edit") => edit::edit_today_log(&config),
        Some("-t") | Some("--time") => add::handle_with_time(args, &config),
        Some(other) => add::handle_plain_entry(other.to_string(), args, &config),
        None => {
            eprintln!("Bruk: logg [-t hh:mm] <setning> | -l | -n <dager> | -u | -e");
            std::process::exit(1);
        }
    }
}

