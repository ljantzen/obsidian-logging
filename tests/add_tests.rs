use chrono::{Local, NaiveTime};
use std::fs;
use tempfile::TempDir;
use obsidian_logging::config::{Config, ListType, TimeFormat};
use obsidian_logging::commands::add::{handle_plain_entry_with_time, handle_with_time};
use obsidian_logging::utils::{get_log_path_for_date, extract_log_entries};
use std::fs::{create_dir_all, read_to_string, write};

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
        time_label: "Tidspunkt".to_string(),
        event_label: "Hendelse".to_string(),
        category_headers: std::collections::HashMap::new(),
        phrases: std::collections::HashMap::new(),
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
    handle_plain_entry_with_time(vec!["Test entry".to_string()], Some(time), &config, false, None);

    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("* 14:30 Test entry"));

    // Test with 12-hour format
    config.time_format = TimeFormat::Hour12;
    let time = NaiveTime::from_hms_opt(14, 30, 0).unwrap();
    handle_plain_entry_with_time(vec!["Another test".to_string()], Some(time), &config, false, None);

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
    handle_with_time(args.into_iter(), &config, false, None);

    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("* 14:30 Test entry"));

    // Test with 12-hour format and 24-hour time input
    config.time_format = TimeFormat::Hour12;
    let args = vec!["14:30".to_string(), "Another".to_string(), "test".to_string()];
    handle_with_time(args.into_iter(), &config, false, None);

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
    handle_plain_entry_with_time(vec!["Test entry".to_string()], Some(time), &config, false, None);

    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("| Tidspunkt | Hendelse |"));
    assert!(content.contains("| 14:30 | Test entry |"));

    // Test with 12-hour format and table
    config.time_format = TimeFormat::Hour12;
    config.list_type = ListType::Table;
    let time = NaiveTime::from_hms_opt(14, 30, 0).unwrap();
    handle_plain_entry_with_time(vec!["Another test".to_string()], Some(time), &config, false, None);

    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("| 02:30 PM | Another test |"));
}

#[test]
fn test_add_with_bullet_format() {
    let (_temp_dir, mut config) = setup_test_env();
    config.list_type = ListType::Bullet;
    
    let now = Local::now();
    let log_path = get_log_path_for_date(now.date_naive(), &config);
    
    // Create directory if it doesn't exist
    if let Some(parent) = log_path.parent() {
        create_dir_all(parent).unwrap();
    }
    
    // Write initial content
    let initial_content = "## Test\n\n- 09:00 First entry\n";
    write(&log_path, initial_content).unwrap();
    
    // Add new log entry
    let time = NaiveTime::from_hms_opt(8, 0, 0).unwrap();
    handle_plain_entry_with_time(vec!["Second entry".to_string()], Some(time), &config, false, None);
    
    // Read and verify content
    let content = read_to_string(&log_path).unwrap();
    let (_, _, entries, _) = extract_log_entries(&content, &config.section_header, &config.list_type, &config, false);
    
    assert_eq!(entries.len(), 2);
    assert!(entries[0].contains("Second entry"));
    assert!(entries[1].contains("First entry"));
}

#[test]
fn test_add_with_invalid_time_does_not_lose_first_word() {
    let (temp_dir, config) = setup_test_env();
    let today = Local::now().date_naive();
    let file_path = temp_dir.path().join(format!("{}.md", today));

    // Test with invalid time that should be treated as part of the sentence
    let args = vec!["invalid_time".to_string(), "This".to_string(), "is".to_string(), "a".to_string(), "test".to_string()];
    handle_with_time(args.into_iter(), &config, false, None);

    let content = fs::read_to_string(&file_path).unwrap();
    // Should contain the full sentence including the invalid time
    assert!(content.contains("invalid_time This is a test"));
} 