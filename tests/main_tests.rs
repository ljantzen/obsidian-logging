use std::env;
use std::path::PathBuf;
use std::str::FromStr;
use tempfile::TempDir;
use obsidian_logging::config::{Config, ListType, TimeFormat};
use assert_cmd::cargo;
use std::fs;
use chrono::Datelike;
use serde_yaml;

fn setup_test_env() -> (PathBuf, Config) {
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().to_path_buf();
    
    // Set up environment variables
    if cfg!(windows) {
        env::set_var("APPDATA", config_dir.to_str().unwrap().to_owned());
    } else {
        env::set_var("HOME", config_dir.to_str().unwrap().to_owned());
    }
    env::set_var("OBSIDIAN_VAULT_DIR", temp_dir.path().to_str().unwrap());

    // Create a temporary config file to prevent reading the real config
    let config_dir_path = if cfg!(windows) {
        config_dir.join("obsidian-logging")
    } else {
        config_dir.join(".config").join("obsidian-logging")
    };
    fs::create_dir_all(&config_dir_path).unwrap();
    
    let test_config = Config {
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
    
    let config_path = config_dir_path.join("obsidian-logging.yaml");
    let yaml = serde_yaml::to_string(&test_config).unwrap();
    fs::write(&config_path, yaml).unwrap();

    (config_dir, test_config)
}

#[test]
fn test_time_format_flag() {
    let (_config_dir, mut config) = setup_test_env();

    // Test 12-hour format
    let args = vec![String::from("-f"), String::from("12")];
    let mut args = args.into_iter().peekable();
    if let Some(arg) = args.next() {
        if arg == "-f" {
            let time_format = args.next().unwrap();
            let time_format = TimeFormat::from_str(&time_format).unwrap();
            config = config.with_time_format(time_format);
        }
    }
    assert_eq!(config.time_format, TimeFormat::Hour12);

    // Test 24-hour format
    let args = vec![String::from("-f"), String::from("24")];
    let mut args = args.into_iter().peekable();
    if let Some(arg) = args.next() {
        if arg == "-f" {
            let time_format = args.next().unwrap();
            let time_format = TimeFormat::from_str(&time_format).unwrap();
            config = config.with_time_format(time_format);
        }
    }
    assert_eq!(config.time_format, TimeFormat::Hour24);

    // Test invalid format
    let args = vec![String::from("-f"), String::from("invalid")];
    let mut args = args.into_iter().peekable();
    if let Some(arg) = args.next() {
        if arg == "-f" {
            let time_format = args.next().unwrap();
            assert!(TimeFormat::from_str(&time_format).is_err());
        }
    }
}

#[test]
fn test_time_format_with_back_flag() {
    let (_config_dir, mut config) = setup_test_env();

    // Test that -f flag is processed before -b flag
    let args = vec![
        String::from("-b"), String::from("4"),
        String::from("-f"), String::from("12"),
    ];
    let mut i = 0;
    let mut command = None;
    let mut command_args = Vec::new();

    // First pass: process only format flags
    while i < args.len() {
        match args[i].as_str() {
            "-f" => {
                i += 1;
                let time_format = TimeFormat::from_str(&args[i]).unwrap();
                config = config.with_time_format(time_format);
                i += 1;
            },
            _ => {
                i += 1;
            }
        }
    }

    // Second pass: process command flags
    i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-f" => {
                i += 2;
            },
            "-b" => {
                command = Some("back");
                i += 1;
                if i < args.len() {
                    command_args.push(args[i].clone());
                }
                i += 1;
            },
            _ => {
                i += 1;
            }
        }
    }

    assert_eq!(config.time_format, TimeFormat::Hour12);
    assert_eq!(command, Some("back"));
    assert_eq!(command_args, vec!["4"]);
}

#[test]
fn test_time_option_preserves_all_words() {
    let (temp_dir, _config) = setup_test_env();
    
    let mut cmd = assert_cmd::Command::new(cargo::cargo_bin!("obsidian-logging"));
    cmd.env("OBSIDIAN_VAULT_DIR", &temp_dir);
    let output = cmd
        .args(["--time", "14:30", "This", "is", "a", "test", "entry"])
        .output()
        .unwrap();
    
    assert!(output.status.success());
    
    // Check that the file was created and contains the full entry
    let today = chrono::Local::now().date_naive();
    let year = today.year();
    let month = today.month();
    let day = today.day();
    
    // Use the default file path format from config
    let file_path = temp_dir
        .join("10-Journal")
        .join(year.to_string())
        .join(format!("{:02}", month))
        .join(format!("{}-{:02}-{:02}.md", year, month, day));
    
    assert!(file_path.exists());
    let content = fs::read_to_string(&file_path).unwrap();
    
    // Should contain the full sentence with the specified time
    assert!(content.contains("14:30 This is a test entry"));
}

#[test]
fn test_version_flags() {
    // Get version from Cargo.toml at compile time
    let expected_version = env!("CARGO_PKG_VERSION");
    
    // Test --version flag
    let mut cmd = assert_cmd::Command::new(cargo::cargo_bin!("obsidian-logging"));
    let output = cmd.arg("--version").output().unwrap();
    
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("obsidian-logging"));
    assert!(stdout.contains(expected_version));
    
    // Test -v flag
    let mut cmd = assert_cmd::Command::new(cargo::cargo_bin!("obsidian-logging"));
    let output = cmd.arg("-v").output().unwrap();
    
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("obsidian-logging"));
    assert!(stdout.contains(expected_version));
} 