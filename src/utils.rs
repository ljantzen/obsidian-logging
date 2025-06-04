use chrono::{Datelike, NaiveDate};
use std::path::PathBuf;
use crate::config::{ListType, Config};
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref TIME_PATTERN: Regex = Regex::new(r"^(?:[-*]\s*)?(\d{2}:\d{2})\s*(.+)$").unwrap();
}

/// Build file path for given date and format string from configuration yaml
/// Supported tokens: {year}, {month}, {date}
pub fn get_log_path_for_date(date: NaiveDate, config: &Config) -> PathBuf {
    // Replace tokens in file path 
    let mut relative_path = config.file_path_format.clone();
    relative_path = relative_path.replace("{year}", &format!("{:04}", date.year()));
    relative_path = relative_path.replace("{month}", &format!("{:02}", date.month()));
    relative_path = relative_path.replace("{date}", &date.to_string());

    let vault_dir = config.vault.clone();
    let mut full_path = PathBuf::from(vault_dir);
    full_path.push(relative_path);
    full_path
}

/// Format a table row with given widths for timestamp and entry columns
fn format_table_row(timestamp: &str, entry: &str, time_width: usize, entry_width: usize) -> String {
    format!("| {:<width$} | {:<width2$} |", timestamp, entry, width=time_width, width2=entry_width)
}

/// Format a table separator line with given column widths
fn format_table_separator(time_width: usize, entry_width: usize) -> String {
    format!("|-{:-<width$}-|-{:-<width2$}-|", "", "", width=time_width, width2=entry_width)
}

/// Parse an entry to extract timestamp and content
fn parse_entry(entry: &str) -> (String, String) {
    if let Some(caps) = TIME_PATTERN.captures(entry.trim()) {
        (caps[1].to_string(), caps[2].trim().to_string())
    } else {
        ("".to_string(), entry.trim().to_string())
    }
}

/// Extract log entries from the log section 
/// Returns ( content before log section, content after log section, list of log entries, and detected list type)
/// Section heading retrieved from yaml config 
pub fn extract_log_entries(content: &str, section_header: &str, list_type: &ListType) -> (String, String, Vec<String>, ListType) {
    let lines: Vec<&str> = content.lines().collect();
    if let Some(start) = lines.iter().position(|line| line.trim() == section_header) {
        let mut i = start + 1;
        let mut entries = Vec::new();
        let mut found_list_type = ListType::Bullet; // Default to bullet
        let mut table_started = false;

        while i < lines.len() {
            let line = lines[i].trim();
            
            // Detect if we're in a table section
            if line.starts_with("| Tidspunkt | Hendelse") {
                found_list_type = ListType::Table;
                table_started = true;
                i += 1;
                continue;
            }
            // skip '|----|----|' 
            if table_started && line.starts_with("|") && line.replace(" ", "").chars().all(|c| c == '|' || c == '-') {
                i += 1;
                continue;
            }

            // Collect entries in their original format
            if (line.starts_with("- ") || line.starts_with("* ")) || (line.starts_with("| ") && table_started) {
                entries.push(line.to_string());
            } else if line.starts_with("## ") && line != section_header {
                break;
            } else if line.is_empty() {
                i += 1;
                continue;
            }
            i += 1;
        }

        // Convert entries if needed
        if found_list_type != *list_type {
            entries = match list_type {
                ListType::Table => {
                    // First pass: collect max widths
                    let mut max_time_width = "Tidspunkt".len();
                    let mut max_entry_width = "Hendelse".len();
                    
                    let parsed_entries: Vec<(String, String)> = entries.iter()
                        .map(|e| {
                            if e.starts_with("- ") || e.starts_with("* ") {
                                let content = e.trim_start_matches(|c| c == '-' || c == '*' || c == ' ');
                                parse_entry(content)
                            } else if e.starts_with("| ") {
                                let parts: Vec<&str> = e.split('|').collect();
                                if parts.len() >= 3 {
                                    (parts[1].trim().to_string(), parts[2].trim().to_string())
                                } else {
                                    let content = e.trim_start_matches(|c| c == '|').trim();
                                    parse_entry(content)
                                }
                            } else {
                                parse_entry(e)
                            }
                        })
                        .collect();

                    // Calculate max widths
                    for (time, entry) in &parsed_entries {
                        max_time_width = max_time_width.max(time.len());
                        max_entry_width = max_entry_width.max(entry.len());
                    }

                    // Format table with consistent widths
                    let mut formatted = Vec::new();
                    formatted.push(format_table_row("Tidspunkt", "Hendelse", max_time_width, max_entry_width));
                    formatted.push(format_table_separator(max_time_width, max_entry_width));
                    
                    formatted.extend(parsed_entries.into_iter().map(|(time, entry)| {
                        format_table_row(&time, &entry, max_time_width, max_entry_width)
                    }));
                    
                    formatted
                },
                ListType::Bullet => entries.into_iter()
                    .map(|e| {
                        if e.starts_with("| ") {
                            let parts: Vec<&str> = e.split('|').collect();
                            if parts.len() >= 3 {
                                let time = parts[1].trim();
                                let entry = parts[2].trim();
                                if time.is_empty() {
                                    format!("- {}", entry)
                                } else {
                                    format!("- {} {}", time, entry)
                                }
                            } else {
                                format!("- {}", e.trim_start_matches(|c| c == '|').trim())
                            }
                        } else {
                            e
                        }
                    })
                    .collect(),
            };
        }

        let before = lines[..start].join("\n") + "\n";
        let after = lines[i..].join("\n");
        (before, after, entries, found_list_type)
    } else {
        (content.to_string(), String::new(), vec![], ListType::Bullet)
    }
}

