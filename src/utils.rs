use chrono::{NaiveDate, NaiveTime, Timelike};
use std::path::PathBuf;
use crate::config::{ListType, Config, TimeFormat};
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref TIME_PATTERN: Regex = Regex::new(r"^(?:[-*]\s*)?(\d{2}:\d{2}(?:\s*[AaPp][Mm])?)\s*(.+)$").unwrap();
}

/// Format time according to the specified format (12 or 24 hour)
pub fn format_time(time: NaiveTime, format: &TimeFormat) -> String {
    match format {
        TimeFormat::Hour24 => time.format("%H:%M").to_string(),
        TimeFormat::Hour12 => {
            let hour = time.hour();
            let minute = time.minute();
            let period = if hour < 12 { "AM" } else { "PM" };
            let hour12 = match hour {
                0 => 12,
                13..=23 => hour - 12,
                _ => hour,
            };
            format!("{:02}:{:02} {}", hour12, minute, period)
        }
    }
}

/// Parse time string in either 12 or 24 hour format
pub fn parse_time(time_str: &str) -> Option<NaiveTime> {
    // Try 24-hour format first
    if let Ok(time) = NaiveTime::parse_from_str(time_str, "%H:%M") {
        return Some(time);
    }

    // Try various 12-hour formats
    let formats = vec![
        "%I:%M %p",    // "02:30 PM"
        "%I:%M%p",     // "02:30PM"
        "%l:%M %p",    // "2:30 PM"
        "%l:%M%p",     // "2:30PM"
    ];

    for format in formats {
        if let Ok(time) = NaiveTime::parse_from_str(&time_str.to_uppercase(), format) {
            return Some(time);
        }
    }

    None
}

/// Build file path for given date and format string from configuration yaml
/// Supported tokens: {year}, {month}, {date}
pub fn get_log_path_for_date(date: NaiveDate, config: &Config) -> PathBuf {
    let mut path = PathBuf::from(&config.vault);
    
    let year = date.format("%Y").to_string();
    let month = date.format("%m").to_string();
    let date_str = date.format("%Y-%m-%d").to_string();
    
    let file_path = config.file_path_format
        .replace("{year}", &year)
        .replace("{month}", &month)
        .replace("{date}", &date_str);
    
    path.push(file_path);
    path
}

/// Format a table row with given widths for timestamp and entry columns
fn format_table_row(timestamp: &str, entry: &str, time_width: usize, entry_width: usize) -> String {
    format!("| {:<width_t$} | {:<width_e$} |",
            timestamp, entry,
            width_t = time_width,
            width_e = entry_width)
}

/// Format a table separator line with given column widths
fn format_table_separator(time_width: usize, entry_width: usize) -> String {
    format!("|{}|{}|",
            "-".repeat(time_width + 2),
            "-".repeat(entry_width + 2))
}

/// Parse an entry to extract timestamp and content
fn parse_entry(entry: &str) -> (String, String) {
    if entry.starts_with('|') {
        // Parse table format
        let parts: Vec<&str> = entry.split('|').collect();
        if parts.len() >= 4 {
            return (parts[1].trim().to_string(), parts[2].trim().to_string());
        }
    } else if entry.starts_with(['*', '-']) {
        // Parse bullet format
        if let Some(space_pos) = entry.find(' ') {
            let content = &entry[space_pos + 1..];
            if let Some(second_space) = content.find(' ') {
                return (content[..second_space].trim().to_string(),
                       content[second_space + 1..].trim().to_string());
            }
        }
    }
    (String::new(), String::new())
}

/// Extract log entries from the log section 
/// Returns ( content before log section, content after log section, list of log entries, and detected list type)
/// Section heading retrieved from yaml config 
pub fn extract_log_entries(content: &str, section_header: &str, list_type: &ListType, config: &Config) -> (String, String, Vec<String>, ListType) {
    let mut before = String::new();
    let mut after = String::new();
    let mut entries = Vec::new();
    let mut found_type = list_type.clone();
    let mut in_section = false;
    let mut found_section = false;

    let mut lines = content.lines().peekable();
    while let Some(line) = lines.next() {
        if line.starts_with(section_header) {
            found_section = true;
            in_section = true;
            before = before.trim_end().to_string() + "\n\n";
            continue;
        }

        if in_section {
            if line.starts_with("##") {
                in_section = false;
                after = line.to_string();
                continue;
            }

            let trimmed = line.trim();
            if !trimmed.is_empty() {
                if trimmed.starts_with('|') {
                    found_type = ListType::Table;
                } else if trimmed.starts_with(['*', '-']) {
                    found_type = ListType::Bullet;
                }

                // Skip table header and separator
                if !trimmed.contains("---") && trimmed != format!("| {} | {} |", config.time_label, config.event_label) {
                    entries.push(line.to_string());
                }
            }
        } else if !found_section {
            before.push_str(line);
            before.push('\n');
        } else if !line.is_empty() {
            after.push('\n');
            after.push_str(line);
        }
    }

    // Convert entries if needed
    if found_type != *list_type {
        let mut converted_entries = Vec::new();
        
        if *list_type == ListType::Table {
            // Convert from bullet to table
            let mut max_time_width = config.time_label.len();
            let mut max_entry_width = config.event_label.len();

            // First pass: calculate widths
            for entry in &entries {
                let (time, text) = parse_entry(entry);
                max_time_width = max_time_width.max(time.len());
                max_entry_width = max_entry_width.max(text.len());
            }

            // Add header
            converted_entries.push(format_table_row(&config.time_label, &config.event_label, max_time_width, max_entry_width));
            converted_entries.push(format_table_separator(max_time_width, max_entry_width));

            // Second pass: format entries
            for entry in entries {
                let (time, text) = parse_entry(&entry);
                converted_entries.push(format_table_row(&time, &text, max_time_width, max_entry_width));
            }
        } else {
            // Convert from table to bullet
            for entry in entries {
                let (time, text) = parse_entry(&entry);
                if !time.is_empty() && !text.is_empty() {
                    converted_entries.push(format!("- {} {}", time, text));
                }
            }
        }

        entries = converted_entries;
    }

    (before, after, entries, found_type)
}

