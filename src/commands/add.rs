use chrono::{Local, Timelike, NaiveTime};
use std::fs::{create_dir_all, read_to_string, write};
use crate::config::Config;
use crate::utils::{get_log_path_for_date, extract_log_entries};

pub fn handle_with_time(mut args: impl Iterator<Item=String>, config: &Config) {
    let time_str = args.next().unwrap_or_else(|| {
        eprintln!("Error: -t/--time needs a timestamp with the format HH:mm");
        std::process::exit(1);
    });

    let time_override = Some(NaiveTime::parse_from_str(&time_str, "%H:%M").unwrap_or_else(|_| {
        eprintln!("Error: invalid timestamp '{}'. Use the format HH:mm.", time_str);
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
    let time_str = format!("{:02}:{:02}", time.hour(), time.minute());

    let file_path = get_log_path_for_date(date, config);
    create_dir_all(file_path.parent().unwrap()).expect("Could not create log directory");

    let mut content = read_to_string(&file_path).unwrap_or_default();

    if !content.contains(&config.section_header) {
        content.push_str(&format!("\n{}\n\n", &config.section_header));
    }

    let (before_log, after_log, mut entries) = extract_log_entries(&content, &config.section_header);

    let new_entry = format!("* {} {}", time_str, sentence);
    entries.push(new_entry);

    // Sort log statements chronologically
    entries.sort_by_key(|entry| entry[2..7].to_string());

    let new_content = format!(
        "{}{}{}\n\n{}{}",
        before_log,
        &config.section_header,
        "\n\n",
        entries.join("\n"),
        if after_log.is_empty() {
            String::new()
        } else {
            format!("\n{}", after_log)
        }
    );

    write(&file_path, new_content.trim_end().to_string() + "\n").expect("Error writing logs to file");

    println!("Logged.");
}

