use chrono::{Duration, Local, NaiveDate};
use std::fs::read_to_string;
use crate::config::Config;
use crate::utils::{get_log_path_for_date, extract_log_entries, parse_time, format_time};

pub fn list_log_for_day(days_ago: i64, config: &Config) {
    let target_date = Local::now().date_naive() - Duration::days(days_ago);
    list_log_for_date(target_date, config);
}

pub fn list_relative_day(args: &mut impl Iterator<Item = String>, config: &Config) {
    let b_days: i64 = args.next().unwrap_or_else(|| {
        eprintln!("Error: -b needs a numeric argument (eg -b 1 for yesterday)");
        std::process::exit(1);
    }).parse().unwrap_or_else(|_| {
        eprintln!("Error: -b must be numeric argument (eg -b 1 for yesterday)");
        std::process::exit(1);
    });

    list_log_for_day(b_days, config);
}

fn list_log_for_date(date: NaiveDate, config: &Config) {
    let file_path = get_log_path_for_date(date, config);
    let date_str = date.to_string();

    let content = match read_to_string(&file_path) {
        Ok(c) => c,
        Err(_) => {
            println!("No log/s found for {}", date_str);
            return;
        }
    };

    let (_, _, entries, _) = extract_log_entries(&content, &config.section_header, &config.list_type);

    if entries.is_empty() {
        println!("No log-section ({} ) found for {}", config.section_header, date_str);
    } else {
        println!("{} Log/s for {}:", config.section_header, date_str);
        for entry in entries {
            // If the entry contains a timestamp, try to parse and reformat it
            if let Some(caps) = entry.trim_start_matches(|c| c == '-' || c == '*' || c == ' ' || c == '|')
                .trim()
                .split_once(' ') {
                if let Some(time) = parse_time(caps.0) {
                    let formatted_time = format_time(time, &config.time_format);
                    let formatted_entry = if entry.starts_with('|') {
                        // Table format
                        let parts: Vec<&str> = entry.split('|').collect();
                        if parts.len() >= 3 {
                            format!("| {} | {} |", formatted_time, parts[2].trim())
                        } else {
                            entry
                        }
                    } else {
                        // Bullet format
                        format!("* {} {}", formatted_time, caps.1)
                    };
                    println!("{}", formatted_entry);
                    continue;
                }
            }
            // If we couldn't parse/format the timestamp, print the entry as is
            println!("{}", entry);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;
    use crate::config::{Config, TimeFormat, ListType};

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
    fn test_list_with_time_format() {
        let (temp_dir, mut config) = setup_test_env();
        let today = Local::now().date_naive();
        let file_path = temp_dir.path().join(format!("{}.md", today));

        // Create a test file with mixed time formats
        let content = r#"# Test
## Test
* 09:00 AM First entry
* 14:30 Second entry
* 02:15 PM Third entry
"#;
        fs::write(&file_path, content).unwrap();

        // Test with 24-hour format
        config.time_format = TimeFormat::Hour24;
        list_log_for_day(0, &config);
        // Note: We can't easily test stdout directly, but the code is covered

        // Test with 12-hour format
        config.time_format = TimeFormat::Hour12;
        list_log_for_day(0, &config);
    }

    #[test]
    fn test_list_relative_day_with_time_format() {
        let (temp_dir, mut config) = setup_test_env();
        let yesterday = Local::now().date_naive() - Duration::days(1);
        let file_path = temp_dir.path().join(format!("{}.md", yesterday));

        // Create a test file with mixed time formats
        let content = r#"# Test
## Test
* 09:00 AM First entry
* 14:30 Second entry
* 02:15 PM Third entry
"#;
        fs::write(&file_path, content).unwrap();

        // Test with 24-hour format
        config.time_format = TimeFormat::Hour24;
        let mut args = vec!["1".to_string()].into_iter();
        list_relative_day(&mut args, &config);

        // Test with 12-hour format
        config.time_format = TimeFormat::Hour12;
        let mut args = vec!["1".to_string()].into_iter();
        list_relative_day(&mut args, &config);
    }

    #[test]
    fn test_list_with_table_format() {
        let (temp_dir, mut config) = setup_test_env();
        let today = Local::now().date_naive();
        let file_path = temp_dir.path().join(format!("{}.md", today));

        // Create a test file with table format
        let content = r#"# Test
## Test
| Tidspunkt | Hendelse |
|-----------|----------|
| 09:00 AM | First entry |
| 14:30 | Second entry |
| 02:15 PM | Third entry |
"#;
        fs::write(&file_path, content).unwrap();

        // Test with 24-hour format and table
        config.time_format = TimeFormat::Hour24;
        config.list_type = ListType::Table;
        list_log_for_day(0, &config);

        // Test with 12-hour format and table
        config.time_format = TimeFormat::Hour12;
        config.list_type = ListType::Table;
        list_log_for_day(0, &config);
    }
}

