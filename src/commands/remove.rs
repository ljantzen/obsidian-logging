use chrono::Local;
use std::fs::{read_to_string, write};
use crate::config::Config;
use crate::utils::{get_log_path_for_date, extract_log_entries};

pub fn remove_last_log_entry(config: &Config) {
    let today = Local::now().date_naive();
    let file_path = get_log_path_for_date(today, config);
    let content = match read_to_string(&file_path) {
        Ok(c) => c,
        Err(_) => {
            println!("No log file found for today.");
            return;
        }
    };

    let (before, after, mut entries) = extract_log_entries(&content, &config.layout.section_header);
    if entries.is_empty() {
        println!("Nothing to remove.");
        return;
    }

    let removed = entries.pop().unwrap(); // Safe since we checked that the list isnt empty
    let new_content = format!(
        "{}{}\n\n{}\n{}",
        before,
        config.layout.section_header,
        entries.join("\n"),
        after
    );

    write(&file_path, new_content.trim_end().to_string() + "\n").expect("Could not write log entries back to file");
    println!("Removed the last log entry: {}", removed);
}

