use crate::config::{Config, ListType};
use crate::template::get_template_content;
use crate::utils::{extract_log_entries, format_time, get_log_path_for_date, parse_time};
use chrono::{Duration, Local, NaiveTime, Timelike};
use std::fs::{create_dir_all, read_to_string, write};

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

    // Try to find a valid time pattern at the beginning
    // This handles both 24-hour (HH:MM:SS) and 12-hour (HH:MM:SS AM/PM) formats
    let time_patterns = [
        // 24-hour format: HH:MM:SS
        r"^(\d{1,2}:\d{2}:\d{2})\s+(.+)$",
        // 24-hour format: HH:MM (backward compatibility)
        r"^(\d{1,2}:\d{2})\s+(.+)$",
        // 12-hour format: HH:MM:SS AM/PM
        r"^(\d{1,2}:\d{2}:\d{2}\s+[AaPp][Mm])\s+(.+)$",
        // 12-hour format: HH:MM AM/PM (backward compatibility)
        r"^(\d{1,2}:\d{2}\s+[AaPp][Mm])\s+(.+)$",
    ];

    for pattern in &time_patterns {
        if let Ok(regex) = regex::Regex::new(pattern) {
            if let Some(captures) = regex.captures(content) {
                let time = captures.get(1).unwrap().as_str().trim();
                let entry = captures.get(2).unwrap().as_str().trim();
                return Some((time.to_string(), entry.to_string()));
            }
        }
    }

    // Fallback to original behavior for backward compatibility
    if let Some(space_pos) = content.find(' ') {
        let (time, entry) = content.split_at(space_pos);
        return Some((time.trim().to_string(), entry.trim().to_string()));
    }

    None
}

pub fn handle_with_time(
    mut args: impl Iterator<Item = String>,
    config: &Config,
    silent: bool,
    category: Option<&str>,
) {
    let time_str = args.next().expect("Expected time as first argument");
    let mut sentence_parts = Vec::new();

    // Check if next word is AM/PM
    if let Some(next_word) = args.next() {
        if next_word.eq_ignore_ascii_case("am") || next_word.eq_ignore_ascii_case("pm") {
            let time_with_period = format!("{} {}", time_str, next_word);
            if let Some(time) = parse_time(&time_with_period) {
                sentence_parts.extend(args);
                handle_plain_entry_with_time(sentence_parts, Some(time), config, silent, category);
                return;
            } else {
                // If time parsing failed with AM/PM, treat both as part of the sentence
                sentence_parts.push(time_str);
                sentence_parts.push(next_word);
                sentence_parts.extend(args);
                handle_plain_entry_with_time(sentence_parts, None, config, silent, category);
                return;
            }
        } else {
            sentence_parts.push(next_word);
        }
    }

    // Try parsing time without AM/PM
    if let Some(time) = parse_time(&time_str) {
        sentence_parts.extend(args);
        handle_plain_entry_with_time(sentence_parts, Some(time), config, silent, category);
    } else {
        // If time parsing failed, treat first argument as part of the sentence
        sentence_parts.insert(0, time_str);
        sentence_parts.extend(args);
        handle_plain_entry_with_time(sentence_parts, None, config, silent, category);
    }
}

pub fn handle_plain_entry(
    first_arg: String,
    args: impl Iterator<Item = String>,
    config: &Config,
    silent: bool,
    category: Option<&str>,
) {
    let mut sentence_parts = vec![first_arg];
    sentence_parts.extend(args);
    handle_plain_entry_with_time(sentence_parts, None, config, silent, category);
}

pub fn handle_plain_entry_with_time(
    sentence_parts: Vec<String>,
    time_override: Option<NaiveTime>,
    config: &Config,
    silent: bool,
    category: Option<&str>,
) {
    let sentence = sentence_parts.join(" ");
    let now = Local::now();
    let date = now.date_naive();
    let time = time_override.unwrap_or_else(|| {
        NaiveTime::from_hms_opt(now.hour(), now.minute(), now.second()).unwrap()
    });

    let file_path = get_log_path_for_date(date, config);
    create_dir_all(file_path.parent().unwrap()).expect("Could not create log directory");

    let is_new_file = !file_path.exists();
    let content = if is_new_file {
        get_template_content(config)
    } else {
        read_to_string(&file_path).unwrap_or_default()
    };

    let section_header = config.get_section_header_for_category(category);
    let (before_log, after_log, entries, detected_type) =
        extract_log_entries(&content, section_header, &config.list_type, config, false);

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
    let parsed_entries: Vec<(String, String)> = entries
        .iter()
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

    // Normalize all existing timestamps to the current format for consistent comparison
    // This ensures we can properly detect duplicates even when formats differ
    let normalized_existing: Vec<(NaiveTime, String)> = parsed_entries
        .iter()
        .filter_map(|(time_str, entry)| parse_time(time_str).map(|t| (t, entry.clone())))
        .collect();

    // Find a unique timestamp by incrementing seconds if needed
    let mut final_time = time;

    // Check if timestamp already exists and increment seconds until unique
    // Compare NaiveTime values to handle cases where formats differ
    while normalized_existing
        .iter()
        .any(|(existing_time, _)| *existing_time == final_time)
    {
        // Increment by 1 second using chrono's Duration
        final_time = final_time + Duration::seconds(1);
    }

    // Combine existing entries (with their parsed timestamps) and the new entry,
    // then normalize all to the current format
    let mut all_entries: Vec<(NaiveTime, String)> = normalized_existing;
    all_entries.push((final_time, sentence.clone()));

    // Sort entries by timestamp
    all_entries.sort_by(|a, b| a.0.cmp(&b.0));

    // Normalize all timestamps to include seconds and use current format
    // This ensures existing entries without seconds get reformatted with seconds
    let normalized_entries: Vec<(String, String)> = all_entries
        .iter()
        .map(|(parsed_time, entry)| {
            let normalized_time = format_time(*parsed_time, &config.time_format);
            (normalized_time, entry.clone())
        })
        .collect();

    // Format entries according to effective type
    let formatted_entries = match effective_type {
        ListType::Bullet => normalized_entries
            .into_iter()
            .map(|(time, entry)| format!("* {} {}", time, entry))
            .collect(),
        ListType::Table => {
            // Calculate maximum widths
            let mut max_time_width = config.time_label.len();
            let mut max_entry_width = config.event_label.len();

            for (time, entry) in &normalized_entries {
                max_time_width = max_time_width.max(time.len());
                max_entry_width = max_entry_width.max(entry.len());
            }

            // Format table
            let mut table = Vec::new();
            // Always show header for table format
            table.push(format!(
                "| {} | {} |",
                config.time_label, config.event_label
            ));
            table.push(format!(
                "| {} | {} |",
                "-".repeat(max_time_width),
                "-".repeat(max_entry_width)
            ));
            table.extend(
                normalized_entries
                    .into_iter()
                    .map(|(time, entry)| format!("| {} | {} |", time, entry)),
            );
            table
        }
    };

    let new_content = format!(
        "{}{}\n\n{}\n{}",
        before_log,
        section_header,
        formatted_entries.join("\n"),
        if after_log.is_empty() {
            String::new()
        } else {
            format!("\n{}", after_log)
        }
    );

    write(&file_path, new_content.trim_end().to_string() + "\n")
        .expect("Error writing logs to file");

    if !silent {
        println!("Logged.");
    }
}
