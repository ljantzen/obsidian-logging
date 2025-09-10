use crate::config::Config;
use crate::utils::{extract_log_entries, get_log_path_for_date};
use chrono::{Duration, Local};
use std::fs::read_to_string;

pub fn list_log_for_day(relative_day: i64, config: &Config, silent: bool, include_header: bool) {
    let date = Local::now().date_naive() - Duration::days(relative_day);
    let log_path = get_log_path_for_date(date, config);

    if !log_path.exists() {
        if !silent {
            println!("No log found for {}", date);
        }
        return;
    }

    let content = read_to_string(&log_path).unwrap_or_else(|_| {
        if !silent {
            println!("Error reading log file");
        }
        String::new()
    });

    let (_, _, entries, _) =
        extract_log_entries(&content, &config.section_header, &config.list_type, config, include_header);

    if entries.is_empty() {
        if !silent {
            println!("No entries found for {}", date);
        }
        return;
    }

    if !silent {
        println!("Log entries for {}:", date);
        for entry in entries {
            println!("{}", entry);
        }
    }
}

