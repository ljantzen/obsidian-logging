use chrono::{Duration, Local};
use obsidian_logging::commands::list::list_log_for_day;
use obsidian_logging::config::{Config, ListType, TimeFormat};
use std::fs;
use tempfile::TempDir;

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
    list_log_for_day(0, &config, false, false, &[]);
    // Note: We can't easily test stdout directly, but the code is covered

    // Test with 12-hour format
    config.time_format = TimeFormat::Hour12;
    list_log_for_day(0, &config, false, false, &[]);
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
    list_log_for_day(0, &config, false, false, &[]);

    // Test with 12-hour format and table
    config.time_format = TimeFormat::Hour12;
    config.list_type = ListType::Table;
    list_log_for_day(0, &config, false, false, &[]);
}

#[test]
fn test_list_past_date() {
    let (temp_dir, config) = setup_test_env();
    let two_days_ago = Local::now().date_naive() - Duration::days(2);
    let file_path = temp_dir.path().join(format!("{}.md", two_days_ago));

    // Create a test file
    let content = r#"# Test
## Test
* 09:00 First entry
* 14:30 Second entry
"#;
    fs::write(&file_path, content).unwrap();

    // Test listing a past date
    list_log_for_day(2, &config, false, false, &[]);
    // Note: We can't easily test stdout directly, but the code is covered
}

#[test]
fn test_list_future_date() {
    let (temp_dir, config) = setup_test_env();
    let tomorrow = Local::now().date_naive() + Duration::days(1);
    let file_path = temp_dir.path().join(format!("{}.md", tomorrow));

    // Create a test file
    let content = r#"# Test
## Test
* 09:00 First entry
* 14:30 Second entry
"#;
    fs::write(&file_path, content).unwrap();

    // Test listing a future date
    list_log_for_day(-1, &config, false, false, &[]);
    // Note: We can't easily test stdout directly, but the code is covered
}

#[test]
fn test_list_nonexistent_date() {
    let (temp_dir, config) = setup_test_env();
    let two_days_ago = Local::now().date_naive() - Duration::days(2);
    let file_path = temp_dir.path().join(format!("{}.md", two_days_ago));

    // Ensure the file doesn't exist
    if file_path.exists() {
        fs::remove_file(&file_path).unwrap();
    }

    // Test listing a non-existent date
    list_log_for_day(2, &config, false, false, &[]);
    // Note: We can't easily test stdout directly, but the code is covered
}
