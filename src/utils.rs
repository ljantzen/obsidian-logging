use chrono::{Datelike, NaiveDate, NaiveTime, Timelike};
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
        TimeFormat::Hour12 => {
            let hour = time.hour();
            let minute = time.minute();
            let period = if hour < 12 { "AM" } else { "PM" };
            let hour12 = if hour == 0 { 12 } else if hour > 12 { hour - 12 } else { hour };
            format!("{:02}:{:02} {}", hour12, minute, period)
        }
        TimeFormat::Hour24 => {
            format!("{:02}:{:02}", time.hour(), time.minute())
        }
    }
}

/// Parse time string in either 12 or 24 hour format
pub fn parse_time(time_str: &str) -> Option<NaiveTime> {
    // Try 24-hour format first
    if let Ok(time) = NaiveTime::parse_from_str(time_str, "%H:%M") {
        return Some(time);
    }

    // Try 12-hour format with various patterns
    let time_str = time_str.trim().to_uppercase();
    let patterns = [
        "%I:%M %p",
        "%I:%M%p",
        "%l:%M %p",
        "%l:%M%p",
    ];

    for pattern in patterns {
        if let Ok(time) = NaiveTime::parse_from_str(&time_str, pattern) {
            return Some(time);
        }
    }

    None
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
                }
                ListType::Bullet => {
                    entries.into_iter()
                        .filter_map(|e| {
                            if e.starts_with("| ") {
                                let parts: Vec<&str> = e.split('|').collect();
                                if parts.len() >= 3 {
                                    Some(format!("- {} {}", parts[1].trim(), parts[2].trim()))
                                } else {
                                    None
                                }
                            } else {
                                Some(e)
                            }
                        })
                        .collect()
                }
            };
        }

        let before = lines[..start].join("\n") + "\n";
        let after = if i < lines.len() {
            lines[i..].join("\n")
        } else {
            String::new()
        };

        (before, after, entries, found_list_type)
    } else {
        (content.to_string(), String::new(), Vec::new(), ListType::Bullet)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn create_test_config() -> Config {
        Config {
            vault: "/test/vault".to_string(),
            file_path_format: "test/{year}/{month}/{date}.md".to_string(),
            section_header: "## Test".to_string(),
            list_type: ListType::Bullet,
            template_path: None,
            locale: None,
            time_format: TimeFormat::Hour24,
        }
    }

    #[test]
    fn test_get_log_path_for_date() {
        let config = create_test_config();
        let date = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
        
        let path = get_log_path_for_date(date, &config);
        let mut expected_path = PathBuf::from("/test/vault");
        expected_path.push("test");
        expected_path.push("2024");
        expected_path.push("03");
        expected_path.push("2024-03-15.md");
        
        assert_eq!(path, expected_path);
    }

    #[test]
    fn test_extract_log_entries_bullet() {
        let content = r#"# Header
Some content

## Test
* 09:00 First entry
* 10:30 Second entry
* 11:15 Third entry

## Another section"#;

        let config = create_test_config();
        let (before, after, entries, found_type) = extract_log_entries(content, &config.section_header, &ListType::Bullet);

        assert_eq!(before, "# Header\nSome content\n\n");
        assert_eq!(after, "## Another section");
        assert_eq!(entries, vec![
            "* 09:00 First entry",
            "* 10:30 Second entry",
            "* 11:15 Third entry"
        ]);
        assert_eq!(found_type, ListType::Bullet);
    }

    #[test]
    fn test_extract_log_entries_table() {
        let content = r#"# Header
Some content

## Test
| Tidspunkt | Hendelse |
|-----------|----------|
| 09:00 | First entry |
| 10:30 | Second entry |
| 11:15 | Third entry |

## Another section"#;

        let config = create_test_config();
        let (before, after, entries, found_type) = extract_log_entries(content, &config.section_header, &ListType::Table);

        assert_eq!(before, "# Header\nSome content\n\n");
        assert_eq!(after, "## Another section");
        assert_eq!(entries, vec![
            "| 09:00 | First entry |",
            "| 10:30 | Second entry |",
            "| 11:15 | Third entry |"
        ]);
        assert_eq!(found_type, ListType::Table);
    }

    #[test]
    fn test_extract_log_entries_empty() {
        let content = r#"# Header
Some content

## Test

## Another section"#;

        let config = create_test_config();
        let (before, after, entries, found_type) = extract_log_entries(content, &config.section_header, &ListType::Bullet);

        assert_eq!(before, "# Header\nSome content\n\n");
        assert_eq!(after, "## Another section");
        assert!(entries.is_empty());
        assert_eq!(found_type, ListType::Bullet);
    }

    #[test]
    fn test_extract_log_entries_no_section() {
        let content = "# Header\nSome content\n";

        let config = create_test_config();
        let (before, after, entries, found_type) = extract_log_entries(content, &config.section_header, &ListType::Bullet);

        assert_eq!(before, content);
        assert_eq!(after, "");
        assert!(entries.is_empty());
        assert_eq!(found_type, ListType::Bullet);
    }

    #[test]
    fn test_extract_log_entries_convert_bullet_to_table() {
        let content = r#"## Test
* 09:00 First entry
* 10:30 Second entry"#;

        let config = create_test_config();
        let (_, _, entries, _) = extract_log_entries(content, &config.section_header, &ListType::Table);

        // Should convert to table format with consistent column widths
        assert_eq!(entries[0], "| Tidspunkt | Hendelse     |");
        assert_eq!(entries[1], "|-----------|--------------|");
        assert_eq!(entries[2], "| 09:00     | First entry  |");
        assert_eq!(entries[3], "| 10:30     | Second entry |");
    }

    #[test]
    fn test_extract_log_entries_convert_table_to_bullet() {
        let content = r#"## Test
| Tidspunkt | Hendelse |
|-----------|----------|
| 09:00 | First entry |
| 10:30 | Second entry |"#;

        let config = create_test_config();
        let (_, _, entries, _) = extract_log_entries(content, &config.section_header, &ListType::Bullet);

        // Should convert to bullet format
        assert_eq!(entries[0], "- 09:00 First entry");
        assert_eq!(entries[1], "- 10:30 Second entry");
    }

    #[test]
    fn test_extract_log_entries_table_format() {
        let content = r#"# Header
Some content

## Test
| Tidspunkt | Hendelse |
|-----------|----------|
| 09:00 | First entry |
| 10:30 | Second entry |
| 11:15 | Third entry |

## Another section"#;

        let config = create_test_config();
        let (before, after, entries, found_type) = extract_log_entries(content, &config.section_header, &ListType::Table);

        assert_eq!(before, "# Header\nSome content\n\n");
        assert_eq!(after, "## Another section");
        assert_eq!(entries, vec![
            "| 09:00 | First entry |",
            "| 10:30 | Second entry |",
            "| 11:15 | Third entry |"
        ]);
        assert_eq!(found_type, ListType::Table);
    }

    #[test]
    fn test_format_time_24h() {
        let time = NaiveTime::from_hms_opt(14, 30, 0).unwrap();
        let formatted = format_time(time, &TimeFormat::Hour24);
        assert_eq!(formatted, "14:30");
    }

    #[test]
    fn test_format_time_12h() {
        let test_cases = vec![
            (0, 30, "12:30 AM"),
            (1, 30, "01:30 AM"),
            (11, 30, "11:30 AM"),
            (12, 30, "12:30 PM"),
            (13, 30, "01:30 PM"),
            (23, 30, "11:30 PM"),
        ];

        for (hour, minute, expected) in test_cases {
            let time = NaiveTime::from_hms_opt(hour, minute, 0).unwrap();
            let formatted = format_time(time, &TimeFormat::Hour12);
            assert_eq!(formatted, expected);
        }
    }

    #[test]
    fn test_parse_time() {
        // Test 24-hour format
        assert_eq!(
            parse_time("14:30"),
            Some(NaiveTime::from_hms_opt(14, 30, 0).unwrap())
        );

        // Test 12-hour format with various formats
        let test_cases = vec![
            "02:30 PM",
            "02:30PM",
            "02:30 pm",
            "02:30pm",
            "2:30 PM",
            "2:30PM",
        ];

        for time_str in test_cases {
            assert_eq!(
                parse_time(time_str),
                Some(NaiveTime::from_hms_opt(14, 30, 0).unwrap()),
                "Failed to parse {}",
                time_str
            );
        }

        // Test invalid formats
        assert_eq!(parse_time("not a time"), None);
        assert_eq!(parse_time("25:00"), None);
        assert_eq!(parse_time("14:60"), None);
        assert_eq!(parse_time("02:30 MP"), None);
    }

    #[test]
    fn test_extract_log_entries_with_time_formats() {
        // Test with mixed 12/24 hour formats
        let content = r#"# Header
Some content

## Test
* 09:00 AM First entry
* 14:30 Second entry
* 02:15 PM Third entry

## Another section"#;

        let config = create_test_config();
        
        // Test with 24-hour format
        let config_24h = Config {
            time_format: TimeFormat::Hour24,
            ..config.clone()
        };
        let (_, _, entries, _) = extract_log_entries(content, &config_24h.section_header, &config_24h.list_type);
        assert_eq!(entries, vec![
            "* 09:00 First entry",
            "* 14:30 Second entry",
            "* 14:15 Third entry"
        ]);

        // Test with 12-hour format
        let config_12h = Config {
            time_format: TimeFormat::Hour12,
            ..config
        };
        let (_, _, entries, _) = extract_log_entries(content, &config_12h.section_header, &config_12h.list_type);
        assert_eq!(entries, vec![
            "* 09:00 AM First entry",
            "* 02:30 PM Second entry",
            "* 02:15 PM Third entry"
        ]);
    }
}

