use crate::config::Config;
use crate::utils::{extract_log_entries, get_log_path_for_date};
use chrono::{Duration, Local};
use std::fs::read_to_string;

pub fn list_log_for_day(relative_day: i64, config: &Config) {
    let date = Local::now().date_naive() - Duration::days(relative_day);
    let log_path = get_log_path_for_date(date, config);

    if !log_path.exists() {
        println!("No log found for {}", date);
        return;
    }

    let content = read_to_string(&log_path).unwrap_or_else(|_| {
        println!("Error reading log file");
        String::new()
    });

    let (_, _, entries, _) =
        extract_log_entries(&content, &config.section_header, &config.list_type, config);

    if entries.is_empty() {
        println!("No entries found for {}", date);
        return;
    }

    println!("Log entries for {}:", date);
    for entry in entries {
        println!("{}", entry);
    }
}

