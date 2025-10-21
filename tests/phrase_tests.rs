use std::fs;
use tempfile::TempDir;
use obsidian_logging::config::{Config, ListType, TimeFormat};
use std::collections::HashMap;
use assert_cmd::prelude::*;
use std::process::Command;

fn setup_test_env_with_phrases() -> (TempDir, Config) {
    let temp_dir = TempDir::new().unwrap();
    let mut phrases = HashMap::new();
    phrases.insert("meeting".to_string(), "Team meeting with stakeholders".to_string());
    phrases.insert("gym".to_string(), "Workout at the gym".to_string());
    phrases.insert("lunch".to_string(), "Lunch break".to_string());
    
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
        category_headers: HashMap::new(),
        phrases,
    };
    (temp_dir, config)
}

#[test]
fn test_phrase_expansion_basic() {
    let (temp_dir, config) = setup_test_env_with_phrases();
    
    // Create the config file
    let config_dir = if cfg!(windows) {
        temp_dir.path().join("obsidian-logging")
    } else {
        temp_dir.path().join(".config").join("obsidian-logging")
    };
    fs::create_dir_all(&config_dir).unwrap();
    
    let config_path = config_dir.join("obsidian-logging.yaml");
    let yaml = serde_yaml::to_string(&config).unwrap();
    fs::write(&config_path, yaml).unwrap();
    
    // Set environment variables
    std::env::set_var("OBSIDIAN_VAULT_DIR", temp_dir.path().to_str().unwrap());
    if cfg!(windows) {
        std::env::set_var("APPDATA", temp_dir.path().to_str().unwrap());
    } else {
        std::env::set_var("HOME", temp_dir.path().to_str().unwrap());
    }
    
    // Test phrase expansion
    let mut cmd = Command::cargo_bin("obsidian-logging").unwrap();
    cmd.args(&["-p", "meeting"]);
    
    let output = cmd.output().unwrap();
    assert!(output.status.success());
    
    // Check that the phrase was expanded in the log file
    let today = chrono::Local::now().date_naive();
    let file_path = temp_dir.path().join(format!("{}.md", today));
    assert!(file_path.exists());
    
    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("Team meeting with stakeholders"));
}

#[test]
fn test_phrase_expansion_with_category() {
    let (temp_dir, config) = setup_test_env_with_phrases();
    
    // Create the config file
    let config_dir = if cfg!(windows) {
        temp_dir.path().join("obsidian-logging")
    } else {
        temp_dir.path().join(".config").join("obsidian-logging")
    };
    fs::create_dir_all(&config_dir).unwrap();
    
    let config_path = config_dir.join("obsidian-logging.yaml");
    let yaml = serde_yaml::to_string(&config).unwrap();
    fs::write(&config_path, yaml).unwrap();
    
    // Set environment variables
    std::env::set_var("OBSIDIAN_VAULT_DIR", temp_dir.path().to_str().unwrap());
    if cfg!(windows) {
        std::env::set_var("APPDATA", temp_dir.path().to_str().unwrap());
    } else {
        std::env::set_var("HOME", temp_dir.path().to_str().unwrap());
    }
    
    // Test phrase expansion with category
    let mut cmd = Command::cargo_bin("obsidian-logging").unwrap();
    cmd.args(&["-p", "gym", "-c", "health"]);
    
    let output = cmd.output().unwrap();
    assert!(output.status.success());
    
    // Check that the phrase was expanded in the log file
    let today = chrono::Local::now().date_naive();
    let file_path = temp_dir.path().join(format!("{}.md", today));
    assert!(file_path.exists());
    
    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("Workout at the gym"));
}

#[test]
fn test_phrase_expansion_with_time() {
    let (temp_dir, config) = setup_test_env_with_phrases();
    
    // Create the config file
    let config_dir = if cfg!(windows) {
        temp_dir.path().join("obsidian-logging")
    } else {
        temp_dir.path().join(".config").join("obsidian-logging")
    };
    fs::create_dir_all(&config_dir).unwrap();
    
    let config_path = config_dir.join("obsidian-logging.yaml");
    let yaml = serde_yaml::to_string(&config).unwrap();
    fs::write(&config_path, yaml).unwrap();
    
    // Set environment variables
    std::env::set_var("OBSIDIAN_VAULT_DIR", temp_dir.path().to_str().unwrap());
    if cfg!(windows) {
        std::env::set_var("APPDATA", temp_dir.path().to_str().unwrap());
    } else {
        std::env::set_var("HOME", temp_dir.path().to_str().unwrap());
    }
    
    // Test phrase expansion with time
    let mut cmd = Command::cargo_bin("obsidian-logging").unwrap();
    cmd.args(&["-p", "lunch", "-t", "12:30"]);
    
    let output = cmd.output().unwrap();
    assert!(output.status.success());
    
    // Check that the phrase was expanded in the log file
    let today = chrono::Local::now().date_naive();
    let file_path = temp_dir.path().join(format!("{}.md", today));
    assert!(file_path.exists());
    
    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("Lunch break"));
    assert!(content.contains("12:30"));
}

#[test]
fn test_phrase_not_found() {
    let (temp_dir, config) = setup_test_env_with_phrases();
    
    // Create the config file
    let config_dir = if cfg!(windows) {
        temp_dir.path().join("obsidian-logging")
    } else {
        temp_dir.path().join(".config").join("obsidian-logging")
    };
    fs::create_dir_all(&config_dir).unwrap();
    
    let config_path = config_dir.join("obsidian-logging.yaml");
    let yaml = serde_yaml::to_string(&config).unwrap();
    fs::write(&config_path, yaml).unwrap();
    
    // Set environment variables
    std::env::set_var("OBSIDIAN_VAULT_DIR", temp_dir.path().to_str().unwrap());
    if cfg!(windows) {
        std::env::set_var("APPDATA", temp_dir.path().to_str().unwrap());
    } else {
        std::env::set_var("HOME", temp_dir.path().to_str().unwrap());
    }
    
    // Test phrase not found
    let mut cmd = Command::cargo_bin("obsidian-logging").unwrap();
    cmd.args(&["-p", "nonexistent"]);
    
    let output = cmd.output().unwrap();
    assert!(!output.status.success());
    
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Phrase 'nonexistent' not found"));
}

#[test]
fn test_config_phrases_loading() {
    let temp_dir = TempDir::new().unwrap();
    let config_dir = if cfg!(windows) {
        temp_dir.path().join("obsidian-logging")
    } else {
        temp_dir.path().join(".config").join("obsidian-logging")
    };
    fs::create_dir_all(&config_dir).unwrap();
    
    // Create a config file with phrases
    let config_content = "
vault: /test/vault
file_path_format: \"{date}.md\"
section_header: \"## Test\"
list_type: bullet
time_format: 24
time_label: \"Time\"
event_label: \"Event\"
phrases:
  meeting: \"Team meeting\"
  gym: \"Workout session\"
  lunch: \"Lunch break\"
";
    
    let config_path = config_dir.join("obsidian-logging.yaml");
    fs::write(&config_path, config_content).unwrap();
    
    // Set environment variables
    std::env::set_var("OBSIDIAN_VAULT_DIR", temp_dir.path().to_str().unwrap());
    if cfg!(windows) {
        std::env::set_var("APPDATA", temp_dir.path().to_str().unwrap());
    } else {
        std::env::set_var("HOME", temp_dir.path().to_str().unwrap());
    }
    
    // Test that phrases are loaded correctly
    let mut cmd = Command::cargo_bin("obsidian-logging").unwrap();
    cmd.args(&["-p", "meeting"]);
    
    let output = cmd.output().unwrap();
    assert!(output.status.success());
    
    // Check that the phrase was expanded
    let today = chrono::Local::now().date_naive();
    let file_path = temp_dir.path().join(format!("{}.md", today));
    assert!(file_path.exists());
    
    let content = fs::read_to_string(&file_path).unwrap();
    assert!(content.contains("Team meeting"));
}
