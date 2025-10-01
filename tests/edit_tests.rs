use std::fs;
use std::env;
use tempfile::TempDir;
use obsidian_logging::config::{Config, ListType, TimeFormat};
use obsidian_logging::commands::edit::edit_log_for_day;
use obsidian_logging::utils::get_log_path_for_date;
use chrono::{Local, Duration};

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
    };
    (temp_dir, config)
}

#[test]
fn test_edit_today() {
    let ( _temp_dir, config) = setup_test_env();
    let today = Local::now().date_naive();
    let file_path = get_log_path_for_date(today, &config);

    // Create parent directory
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    // Create a test file
    let content = r#"# Test
## Test
* 09:00 First entry
* 14:30 Second entry
"#;
    fs::write(&file_path, content).unwrap();

    // Set EDITOR to echo for testing
    env::set_var("EDITOR", "echo");

    // Test editing today's file
    edit_log_for_day(0, &config, false);

    // Verify the file still exists and has the same content
    assert!(file_path.exists());
    let content_after = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, content_after);
}

#[test]
fn test_edit_relative_day() {
    let (_temp_dir, config) = setup_test_env();
    let yesterday = Local::now().date_naive() - Duration::days(1);
    let file_path = get_log_path_for_date(yesterday, &config);

    // Create parent directory
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    // Create a test file
    let content = r#"# Test
## Test
* 09:00 First entry
* 14:30 Second entry
"#;
    fs::write(&file_path, content).unwrap();

    // Set EDITOR to echo for testing
    env::set_var("EDITOR", "echo");

    // Test editing yesterday's file
    edit_log_for_day(1, &config, false);

    // Verify the file still exists and has the same content
    assert!(file_path.exists());
    let content_after = fs::read_to_string(&file_path).unwrap();
    assert_eq!(content, content_after);
}

#[test]
fn test_edit_nonexistent_file() {
    let (_temp_dir, config) = setup_test_env();
    let tomorrow = Local::now().date_naive() + Duration::days(1);
    let file_path = get_log_path_for_date(tomorrow, &config);

    // Create parent directory
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    // Set EDITOR to echo for testing
    env::set_var("EDITOR", "echo");

    // Test editing a non-existent file
    edit_log_for_day(-1, &config, false);  // -1 means tomorrow now

    // Verify the file was created with template content
    assert!(file_path.exists());
    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("## 🕗"));
}

#[test]
fn test_edit_past_date_does_not_create_file() {
    let (_temp_dir, config) = setup_test_env();
    let two_days_ago = Local::now().date_naive() - Duration::days(2);
    let file_path = get_log_path_for_date(two_days_ago, &config);

    // Create parent directory
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    // Set EDITOR to echo for testing
    env::set_var("EDITOR", "echo");

    // Test editing a non-existent past file
    edit_log_for_day(2, &config, false);

    // Verify the file was NOT created
    assert!(!file_path.exists());
}

#[test]
fn test_edit_future_date_creates_file() {
    let (_temp_dir, config) = setup_test_env();
    let tomorrow = Local::now().date_naive() + Duration::days(1);
    let file_path = get_log_path_for_date(tomorrow, &config);

    // Create parent directory
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    // Set EDITOR to echo for testing
    env::set_var("EDITOR", "echo");

    // Test editing a non-existent future file
    edit_log_for_day(-1, &config, false);

    // Verify the file was created with template content
    assert!(file_path.exists());
    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("## 🕗"));
} 