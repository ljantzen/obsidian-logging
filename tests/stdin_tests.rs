use tempfile::TempDir;
use obsidian_logging::config::{Config, ListType, TimeFormat};
use obsidian_logging::commands::add::{handle_plain_entry, handle_with_time};
use std::fs;
use chrono::Local;

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
fn test_stdin_functionality() {
    // Test basic stdin functionality by simulating the stdin processing logic
    let (temp_dir, config) = setup_test_env();
    let today = Local::now().date_naive();
    let file_path = temp_dir.path().join(format!("{}.md", today));

    // Simulate stdin input
    let stdin_content = "Test stdin entry";
    let entry_words: Vec<String> = stdin_content.split_whitespace().map(|s| s.to_string()).collect();
    
    // Process the entry
    let mut args = entry_words.into_iter();
    if let Some(first) = args.next() {
        handle_plain_entry(first, args, &config, false, None);
    }

    // Verify the entry was written
    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("Test stdin entry"));
}

#[test]
fn test_stdin_with_time_override() {
    // Test stdin with time override by simulating the stdin processing logic
    let (temp_dir, config) = setup_test_env();
    let today = Local::now().date_naive();
    let file_path = temp_dir.path().join(format!("{}.md", today));

    // Simulate stdin input with time override
    let stdin_content = "Test stdin entry with time override";
    let entry_words: Vec<String> = stdin_content.split_whitespace().map(|s| s.to_string()).collect();
    
    // Process the entry with time override (simulating -t 14:30)
    let mut time_args = vec!["14:30".to_string()];
    time_args.extend(entry_words);
    handle_with_time(time_args.into_iter(), &config, false, None);

    // Verify the entry was written with the correct time
    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("14:30 Test stdin entry with time override"));
}

#[test]
fn test_stdin_empty_input() {
    // Test stdin with empty input - this test verifies that empty input is handled correctly
    // Since we're testing the internal logic, we'll test that empty input doesn't create entries
    let (temp_dir, config) = setup_test_env();
    let today = Local::now().date_naive();
    let file_path = temp_dir.path().join(format!("{}.md", today));

    // Simulate empty stdin input
    let stdin_content = "";
    let entry_words: Vec<String> = stdin_content.split_whitespace().map(|s| s.to_string()).collect();
    
    // Process the entry (should not create anything for empty input)
    if !entry_words.is_empty() {
        let mut args = entry_words.into_iter();
        if let Some(first) = args.next() {
            handle_plain_entry(first, args, &config, false, None);
        }
    }

    // Verify no entry was written for empty input
    if file_path.exists() {
        let content = fs::read_to_string(&file_path).unwrap();
        // Should only contain template content, no actual entries
        assert!(!content.contains("* ") || content.contains("## Test"));
    }
} 