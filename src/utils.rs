use chrono::{Datelike, NaiveDate};
use std::path::PathBuf;
use crate::config::Config;

/// Build file path for given date and format string from configuration toml
/// Supported tokens: {year}, {month}, {date}
pub fn get_log_path_for_date(date: NaiveDate, config: &Config) -> PathBuf {
    let vault_dir = std::env::var("OBSIDIAN_VAULT").expect("Environment variable $OBSIDIAN_VAULT is undefined");

    // Replace tokens in file path 
    let mut relative_path = config.layout.file_path_format.clone();
    relative_path = relative_path.replace("{year}", &format!("{:04}", date.year()));
    relative_path = relative_path.replace("{month}", &format!("{:02}", date.month()));
    relative_path = relative_path.replace("{date}", &date.to_string());

    let mut full_path = PathBuf::from(vault_dir);
    full_path.push(relative_path);
    full_path
}

/// Extract log entries from the log section 
/// Returns ( content before log section, content after log section and list of log entries)
/// Section heading retrieved from toml config 
pub fn extract_log_entries(content: &str, section_header: &str) -> (String, String, Vec<String>) {
    let lines: Vec<&str> = content.lines().collect();
    if let Some(start) = lines.iter().position(|line| line.trim() == section_header) {
        let mut i = start + 1;
        let mut entries = Vec::new();

        while i < lines.len() {
            let line = lines[i].trim();
            if line.starts_with("* ") {
                entries.push(line.to_string());
            } else if line.starts_with("## ") && line != section_header {
                break;
            }
            i += 1;
        }

        let before = lines[..start].join("\n") + "\n";
        let after = lines[i..].join("\n");
        (before, after, entries)
    } else {
        (content.to_string(), String::new(), vec![])
    }
}

