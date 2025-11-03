use std::fs;
use tempfile::TempDir;
use obsidian_logging::config::{Config, ListType, TimeFormat};
use std::collections::HashMap;
use assert_cmd::cargo;
use std::sync::Mutex;

// Global mutex to ensure environment variable changes are atomic
static ENV_MUTEX: Mutex<()> = Mutex::new(());

/// Helper function to safely set environment variables for testing
/// Returns a guard that will restore the original values when dropped
fn with_test_env<F, R>(temp_dir: &TempDir, f: F) -> R
where
    F: FnOnce() -> R,
{
    let _guard = ENV_MUTEX.lock().unwrap();
    
    // Store original values
    let original_vault = std::env::var("OBSIDIAN_VAULT_DIR").ok();
    let original_home = std::env::var("HOME").ok();
    let original_appdata = std::env::var("APPDATA").ok();
    
    // Set test values
    std::env::set_var("OBSIDIAN_VAULT_DIR", temp_dir.path().to_str().unwrap());
    if cfg!(windows) {
        std::env::set_var("APPDATA", temp_dir.path().to_str().unwrap());
    } else {
        std::env::set_var("HOME", temp_dir.path().to_str().unwrap());
    }
    
    // Run the test
    let result = f();
    
    // Restore original values
    if let Some(vault) = original_vault {
        std::env::set_var("OBSIDIAN_VAULT_DIR", vault);
    } else {
        std::env::remove_var("OBSIDIAN_VAULT_DIR");
    }
    
    if let Some(home) = original_home {
        std::env::set_var("HOME", home);
    } else {
        std::env::remove_var("HOME");
    }
    
    if let Some(appdata) = original_appdata {
        std::env::set_var("APPDATA", appdata);
    } else {
        std::env::remove_var("APPDATA");
    }
    
    result
}

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
    
    with_test_env(&temp_dir, || {
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
        
        // Test phrase expansion
        let mut cmd = assert_cmd::Command::new(cargo::cargo_bin!("obsidian-logging"));
        cmd.args(&["-p", "meeting"]);
        
        let output = cmd.output().unwrap();
        assert!(output.status.success());
        
        // Check that the phrase was expanded in the log file
        let today = chrono::Local::now().date_naive();
        let file_path = temp_dir.path().join(format!("{}.md", today));
        assert!(file_path.exists());
        
        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("Team meeting with stakeholders"));
    });
}

#[test]
fn test_phrase_expansion_with_category() {
    let (temp_dir, config) = setup_test_env_with_phrases();
    
    with_test_env(&temp_dir, || {
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
        
        // Test phrase expansion with category
        let mut cmd = assert_cmd::Command::new(cargo::cargo_bin!("obsidian-logging"));
        cmd.args(&["-p", "gym", "-c", "health"]);
        
        let output = cmd.output().unwrap();
        assert!(output.status.success());
        
        // Check that the phrase was expanded in the log file
        let today = chrono::Local::now().date_naive();
        let file_path = temp_dir.path().join(format!("{}.md", today));
        assert!(file_path.exists());
        
        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("Workout at the gym"));
    });
}

#[test]
fn test_phrase_expansion_with_time() {
    let (temp_dir, config) = setup_test_env_with_phrases();
    
    with_test_env(&temp_dir, || {
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
        
        // Test phrase expansion with time
        let mut cmd = assert_cmd::Command::new(cargo::cargo_bin!("obsidian-logging"));
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
    });
}

#[test]
fn test_phrase_not_found() {
    let (temp_dir, config) = setup_test_env_with_phrases();
    
    with_test_env(&temp_dir, || {
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
        
        // Test phrase not found
        let mut cmd = assert_cmd::Command::new(cargo::cargo_bin!("obsidian-logging"));
        cmd.args(&["-p", "nonexistent"]);
        
        let output = cmd.output().unwrap();
        assert!(!output.status.success());
        
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("Phrase 'nonexistent' not found"));
    });
}

#[test]
fn test_config_phrases_loading() {
    let temp_dir = TempDir::new().unwrap();
    
    with_test_env(&temp_dir, || {
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
        
        // Test that phrases are loaded correctly
        let mut cmd = assert_cmd::Command::new(cargo::cargo_bin!("obsidian-logging"));
        cmd.args(&["-p", "meeting"]);
        
        let output = cmd.output().unwrap();
        assert!(output.status.success());
        
        // Check that the phrase was expanded
        let today = chrono::Local::now().date_naive();
        let file_path = temp_dir.path().join(format!("{}.md", today));
        assert!(file_path.exists());
        
        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("Team meeting"));
    });
}

#[test]
fn test_phrase_argument_expansion_basic() {
    let (temp_dir, config) = setup_test_env_with_phrases();
    
    with_test_env(&temp_dir, || {
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
        
        // Test phrase expansion with arguments
        let mut cmd = assert_cmd::Command::new(cargo::cargo_bin!("obsidian-logging"));
        cmd.args(&["-p", "meeting", "John", "Smith"]);
        
        let output = cmd.output().unwrap();
        assert!(output.status.success());
        
        // Check that the phrase was expanded in the log file
        let today = chrono::Local::now().date_naive();
        let file_path = temp_dir.path().join(format!("{}.md", today));
        assert!(file_path.exists());
        
        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("Team meeting with stakeholders"));
    });
}

#[test]
fn test_phrase_argument_expansion_with_placeholders() {
    let temp_dir = TempDir::new().unwrap();
    let mut phrases = HashMap::new();
    phrases.insert("meeting_with".to_string(), "Team meeting with {*}".to_string());
    phrases.insert("call_with".to_string(), "Phone call with {0}".to_string());
    phrases.insert("project".to_string(), "Working on {0}".to_string());
    
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
    
    with_test_env(&temp_dir, || {
        // Test {*} placeholder expansion
        let mut cmd = assert_cmd::Command::new(cargo::cargo_bin!("obsidian-logging"));
        cmd.args(&["-p", "meeting_with", "John", "Smith"]);
        
        let output = cmd.output().unwrap();
        assert!(output.status.success());
        
        let today = chrono::Local::now().date_naive();
        let file_path = temp_dir.path().join(format!("{}.md", today));
        assert!(file_path.exists());
        
        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("Team meeting with John Smith"));
        
        // Test {0} placeholder expansion
        let mut cmd2 = assert_cmd::Command::new(cargo::cargo_bin!("obsidian-logging"));
        cmd2.args(&["-p", "call_with", "Alice"]);
        
        let output2 = cmd2.output().unwrap();
        assert!(output2.status.success());
        
        let content2 = fs::read_to_string(&file_path).unwrap();
        assert!(content2.contains("Phone call with Alice"));
        
        // Test project placeholder
        let mut cmd3 = assert_cmd::Command::new(cargo::cargo_bin!("obsidian-logging"));
        cmd3.args(&["-p", "project", "Project Alpha"]);
        
        let output3 = cmd3.output().unwrap();
        assert!(output3.status.success());
        
        let content3 = fs::read_to_string(&file_path).unwrap();
        assert!(content3.contains("Working on Project Alpha"));
    });
}

#[test]
fn test_phrase_argument_expansion_with_time() {
    let temp_dir = TempDir::new().unwrap();
    let mut phrases = HashMap::new();
    phrases.insert("meeting_with".to_string(), "Team meeting with {*}".to_string());
    
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
    
    with_test_env(&temp_dir, || {
        // Test phrase expansion with time
        let mut cmd = assert_cmd::Command::new(cargo::cargo_bin!("obsidian-logging"));
        cmd.args(&["-p", "meeting_with", "John", "Smith", "-t", "14:30"]);
        
        let output = cmd.output().unwrap();
        assert!(output.status.success());
        
        let today = chrono::Local::now().date_naive();
        let file_path = temp_dir.path().join(format!("{}.md", today));
        assert!(file_path.exists());
        
        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("Team meeting with John Smith"));
        assert!(content.contains("14:30"));
    });
}

#[test]
fn test_phrase_hash_placeholder_expansion() {
    let temp_dir = TempDir::new().unwrap();
    let mut phrases = HashMap::new();
    phrases.insert("meeting_with".to_string(), "Team meeting with {#}".to_string());
    phrases.insert("call_with".to_string(), "Phone call with {#}".to_string());
    phrases.insert("project_with".to_string(), "Working on {#}".to_string());
    
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
    
    with_test_env(&temp_dir, || {
        // Test {#} placeholder with two items
        let mut cmd = assert_cmd::Command::new(cargo::cargo_bin!("obsidian-logging"));
        cmd.args(&["-p", "meeting_with", "John", "Jane"]);
        
        let output = cmd.output().unwrap();
        assert!(output.status.success());
        
        let today = chrono::Local::now().date_naive();
        let file_path = temp_dir.path().join(format!("{}.md", today));
        assert!(file_path.exists());
        
        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("Team meeting with John and Jane"));
        
        // Test {#} placeholder with three items
        let mut cmd2 = assert_cmd::Command::new(cargo::cargo_bin!("obsidian-logging"));
        cmd2.args(&["-p", "call_with", "Alice", "Bob", "Charlie"]);
        
        let output2 = cmd2.output().unwrap();
        assert!(output2.status.success());
        
        let content2 = fs::read_to_string(&file_path).unwrap();
        assert!(content2.contains("Phone call with Alice, Bob and Charlie"));
        
        // Test {#} placeholder with one item
        let mut cmd3 = assert_cmd::Command::new(cargo::cargo_bin!("obsidian-logging"));
        cmd3.args(&["-p", "project_with", "Frontend"]);
        
        let output3 = cmd3.output().unwrap();
        assert!(output3.status.success());
        
        let content3 = fs::read_to_string(&file_path).unwrap();
        assert!(content3.contains("Working on Frontend"));
    });
}

#[test]
fn test_phrase_hash_placeholder_with_norwegian_conjunction() {
    let temp_dir = TempDir::new().unwrap();
    let mut phrases = HashMap::new();
    phrases.insert("meeting_with".to_string(), "Møte med {#}".to_string());
    
    let config = Config {
        vault: temp_dir.path().to_str().unwrap().to_string(),
        file_path_format: "{date}.md".to_string(),
        section_header: "## Test".to_string(),
        list_type: ListType::Bullet,
        template_path: None,
        locale: Some("no".to_string()),
        time_format: TimeFormat::Hour24,
        time_label: "Tidspunkt".to_string(),
        event_label: "Hendelse".to_string(),
        category_headers: HashMap::new(),
        phrases,
    };
    
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
    
    with_test_env(&temp_dir, || {
        // Test {#} placeholder with Norwegian conjunction
        let mut cmd = assert_cmd::Command::new(cargo::cargo_bin!("obsidian-logging"));
        cmd.args(&["-p", "meeting_with", "John", "Jane"]);
        
        let output = cmd.output().unwrap();
        assert!(output.status.success());
        
        let today = chrono::Local::now().date_naive();
        let file_path = temp_dir.path().join(format!("{}.md", today));
        assert!(file_path.exists());
        
        let content = fs::read_to_string(&file_path).unwrap();
        assert!(content.contains("Møte med John og Jane"));
    });
}
