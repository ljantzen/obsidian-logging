use chrono::{Local, NaiveTime, Timelike};
use std::fs::{create_dir_all, read_to_string, write};
use crate::config::{Config, ListType};
use crate::utils::{get_log_path_for_date, extract_log_entries, format_time, parse_time};
use crate::template::get_template_content;

/// Parse a table row into (timestamp, entry)
fn parse_table_row(line: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() >= 4 {
        let time = parts[1].trim();
        let entry = parts[2].trim();
        if !time.is_empty() && !entry.is_empty() {
            return Some((time.to_string(), entry.to_string()));
        }
    }
    None
}

/// Parse a bullet entry into (timestamp, entry)
fn parse_bullet_entry(line: &str) -> Option<(String, String)> {
    let content = line.trim_start_matches(|c| c == '-' || c == '*' || c == ' ');
    if let Some(space_pos) = content.find(' ') {
        let (time, entry) = content.split_at(space_pos);
        return Some((time.trim().to_string(), entry.trim().to_string()));
    }
    None
}

pub fn handle_with_time(mut args: impl Iterator<Item=String>, config: &Config) {
    let time_str = args.next().expect("Expected time as first argument");
    let mut sentence_parts = Vec::new();

    // Check if next word is AM/PM
    if let Some(next_word) = args.next() {
        if next_word.eq_ignore_ascii_case("am") || next_word.eq_ignore_ascii_case("pm") {
            let time_with_period = format!("{} {}", time_str, next_word);
            if let Some(time) = parse_time(&time_with_period) {
                sentence_parts.extend(args);
                handle_plain_entry_with_time(sentence_parts, Some(time), config);
                return;
            }
        } else {
            sentence_parts.push(next_word);
        }
    }

    // Try parsing time without AM/PM
    if let Some(time) = parse_time(&time_str) {
        sentence_parts.extend(args);
        handle_plain_entry_with_time(sentence_parts, Some(time), config);
    } else {
        // If time parsing failed, treat first argument as part of the sentence
        sentence_parts.insert(0, time_str);
        sentence_parts.extend(args);
        handle_plain_entry_with_time(sentence_parts, None, config);
    }
}

pub fn handle_plain_entry(first_arg: String, args: impl Iterator<Item=String>, config: &Config) {
    let mut sentence_parts = vec![first_arg];
    sentence_parts.extend(args);
    handle_plain_entry_with_time(sentence_parts, None, config);
}

pub fn handle_plain_entry_with_time(sentence_parts: Vec<String>, time_override: Option<NaiveTime>, config: &Config) {
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
            // Always show header for table format
            table.push(format!("| Tidspunkt | Hendelse |"));
            table.push(format!("| --------- | -------- |"));
            table.extend(parsed_entries.into_iter().map(|(time, entry)| {
                format!("| {} | {} |", time, entry)
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



