use chrono::{Local, Timelike, NaiveTime};
use std::fs::{create_dir_all, read_to_string, write};
use crate::config::{Config, ListType};
use crate::utils::{get_log_path_for_date, extract_log_entries, format_time, parse_time};
use crate::template::get_template_content;

/// Format a table row with given widths for timestamp and entry columns
fn format_table_row(timestamp: &str, entry: &str, time_width: usize, entry_width: usize) -> String {
    format!("| {:<width$} | {:<width2$} |", timestamp, entry, width=time_width, width2=entry_width)
}

/// Format a table separator line with given column widths
fn format_table_separator(time_width: usize, entry_width: usize) -> String {
    format!("|-{:-<width$}-|-{:-<width2$}-|", "", "", width=time_width, width2=entry_width)
}

/// Parse a table row into (timestamp, entry)
fn parse_table_row(line: &str) -> Option<(String, String)> {
    // Skip header and separator rows
    if line.contains("Tidspunkt | Hendelse") || line.trim().chars().all(|c| c == '|' || c == '-') {
        return None;
    }
    
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() >= 3 {
        Some((parts[1].trim().to_string(), parts[2].trim().to_string()))
    } else {
        None
    }
}

/// Parse a bullet entry into (timestamp, entry)
fn parse_bullet_entry(line: &str) -> Option<(String, String)> {
    let content = line.trim_start_matches(|c| c == '-' || c == '*' || c == ' ');
    if let Some(space_pos) = content.find(' ') {
        let (time, rest) = content.split_at(space_pos);
        if time.len() == 5 && time.chars().nth(2) == Some(':') {
            return Some((time.to_string(), rest.trim().to_string()));
        }
    }
    None
}

pub fn handle_with_time(mut args: impl Iterator<Item=String>, config: &Config) {
    let time_str = args.next().unwrap_or_else(|| {
        eprintln!("Error: -t/--time needs a timestamp with the format HH:mm or HH:mm AM/PM");
        std::process::exit(1);
    });

    let time_override = Some(parse_time(&time_str).unwrap_or_else(|| {
        eprintln!("Error: invalid timestamp '{}'. Use the format HH:mm or HH:mm AM/PM", time_str);
        std::process::exit(1);
    }));

    let sentence_parts: Vec<String> = args.collect();
    if sentence_parts.is_empty() {
        eprintln!("Error: No log statement provided.");
        std::process::exit(1);
    }

    handle_plain_entry_with_time(sentence_parts, time_override, config);
}

pub fn handle_plain_entry(first_arg: String, args: impl Iterator<Item=String>, config: &Config) {
    let mut sentence_parts = vec![first_arg];
    sentence_parts.extend(args);
    handle_plain_entry_with_time(sentence_parts, None, config);
}

fn handle_plain_entry_with_time(sentence_parts: Vec<String>, time_override: Option<NaiveTime>, config: &Config) {
    let sentence = sentence_parts.join(" ");
    let now = Local::now();
    let date = now.date_naive();
    let time = time_override.unwrap_or_else(|| NaiveTime::from_hms_opt(now.hour(), now.minute(), 0).unwrap());
    let time_str = format_time(time, &config.time_format);

    let file_path = get_log_path_for_date(date, config);
    create_dir_all(file_path.parent().unwrap()).expect("Could not create log directory");

    let is_new_file = !file_path.exists();
    let content = if is_new_file {
        get_template_content(config)
    } else {
        read_to_string(&file_path).unwrap_or_default()
    };

    let (before_log, after_log, entries, detected_type) = extract_log_entries(&content, &config.section_header, &config.list_type);

    // For new files, always use the config list type
    // For existing files, use detected type unless there are no entries
    let effective_type = if is_new_file {
        config.list_type.clone()
    } else if entries.is_empty() {
        config.list_type.clone()
    } else {
        detected_type
    };

    // Parse all entries into (timestamp, entry) pairs
    let mut parsed_entries: Vec<(String, String)> = entries.iter()
        .filter_map(|e| {
            if e.starts_with("| ") {
                parse_table_row(e)
            } else if e.starts_with("- ") || e.starts_with("* ") {
                parse_bullet_entry(e)
            } else {
                None
            }
        })
        .collect();

    // Add the new entry
    parsed_entries.push((time_str, sentence));

    // Sort entries by timestamp (convert to NaiveTime for comparison)
    parsed_entries.sort_by(|a, b| {
        let time_a = parse_time(&a.0).unwrap_or_else(|| NaiveTime::from_hms_opt(0, 0, 0).unwrap());
        let time_b = parse_time(&b.0).unwrap_or_else(|| NaiveTime::from_hms_opt(0, 0, 0).unwrap());
        time_a.cmp(&time_b)
    });

    // Format entries according to effective type
    let formatted_entries = match effective_type {
        ListType::Bullet => {
            parsed_entries.into_iter()
                .map(|(time, entry)| format!("* {} {}", time, entry))
                .collect()
        }
        ListType::Table => {
            // Calculate maximum widths
            let mut max_time_width = "Tidspunkt".len();
            let mut max_entry_width = "Hendelse".len();

            for (time, entry) in &parsed_entries {
                max_time_width = max_time_width.max(time.len());
                max_entry_width = max_entry_width.max(entry.len());
            }

            // Format table
            let mut table = Vec::new();
            // Always show header for table format in new files or when explicitly set
            if is_new_file || effective_type == ListType::Table {
                table.push(format_table_row("Tidspunkt", "Hendelse", max_time_width, max_entry_width));
                table.push(format_table_separator(max_time_width, max_entry_width));
            }
            table.extend(parsed_entries.into_iter().map(|(time, entry)| {
                format_table_row(&time, &entry, max_time_width, max_entry_width)
            }));
            table
        }
    };

    let new_content = format!(
        "{}{}\n\n{}\n{}",
        before_log,
        &config.section_header,
        formatted_entries.join("\n"),
        if after_log.is_empty() {
            String::new()
        } else {
            format!("\n{}", after_log)
        }
    );

    write(&file_path, new_content.trim_end().to_string() + "\n").expect("Error writing logs to file");

    println!("Logged.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;
    use chrono::NaiveTime;

    fn setup_test_env() -> (TempDir, Config) {
        let temp_dir = TempDir::new().unwrap();
        let config = Config {
            vault: temp_dir.path().to_str().unwrap().to_string(),
            file_path_format: "{date}.md".to_string(),
            section_header: "## Test".to_string(),
            list_type: ListType::Bullet,
            template_path: None,
            locale: None,
            time_format: TimeFormat::Hour24,
        };
        (temp_dir, config)
    }

    #[test]
    fn test_add_with_time_format() {
        let (temp_dir, mut config) = setup_test_env();
        let today = Local::now().date_naive();
        let file_path = temp_dir.path().join(format!("{}.md", today));

        // Test with 24-hour format
        config.time_format = TimeFormat::Hour24;
        let time = NaiveTime::from_hms_opt(14, 30, 0).unwrap();
        handle_plain_entry_with_time(vec!["Test entry".to_string()], Some(time), &config);

        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("* 14:30 Test entry"));

        // Test with 12-hour format
        config.time_format = TimeFormat::Hour12;
        let time = NaiveTime::from_hms_opt(14, 30, 0).unwrap();
        handle_plain_entry_with_time(vec!["Another test".to_string()], Some(time), &config);

        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("* 02:30 PM Another test"));
    }

    #[test]
    fn test_add_with_time_override() {
        let (temp_dir, mut config) = setup_test_env();
        let today = Local::now().date_naive();
        let file_path = temp_dir.path().join(format!("{}.md", today));

        // Test with 24-hour format and 12-hour time input
        config.time_format = TimeFormat::Hour24;
        let args = vec!["02:30".to_string(), "PM".to_string(), "Test".to_string(), "entry".to_string()];
        handle_with_time(args.into_iter(), &config);

        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("* 14:30 Test entry"));

        // Test with 12-hour format and 24-hour time input
        config.time_format = TimeFormat::Hour12;
        let args = vec!["14:30".to_string(), "Another".to_string(), "test".to_string()];
        handle_with_time(args.into_iter(), &config);

        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("* 02:30 PM Another test"));
    }

    #[test]
    fn test_add_with_table_format() {
        let (temp_dir, mut config) = setup_test_env();
        let today = Local::now().date_naive();
        let file_path = temp_dir.path().join(format!("{}.md", today));

        // Test with 24-hour format and table
        config.time_format = TimeFormat::Hour24;
        config.list_type = ListType::Table;
        let time = NaiveTime::from_hms_opt(14, 30, 0).unwrap();
        handle_plain_entry_with_time(vec!["Test entry".to_string()], Some(time), &config);

        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("| 14:30 | Test entry |"));

        // Test with 12-hour format and table
        config.time_format = TimeFormat::Hour12;
        config.list_type = ListType::Table;
        let time = NaiveTime::from_hms_opt(14, 30, 0).unwrap();
        handle_plain_entry_with_time(vec!["Another test".to_string()], Some(time), &config);

        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("| 02:30 PM | Another test |"));
    }
}

