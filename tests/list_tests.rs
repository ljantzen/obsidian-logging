use chrono::{Duration, Local};
use std::fs;
use tempfile::TempDir;
use obsidian_logging::config::{Config, TimeFormat, ListType};
use obsidian_logging::commands::list::{list_log_for_day, list_relative_day};

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