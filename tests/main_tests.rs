use std::env;
use std::path::PathBuf;
use std::str::FromStr;
use tempfile::TempDir;
use obsidian_logging::config::{Config, ListType, TimeFormat};

fn setup_test_env() -> (PathBuf, Config) {
    let temp_dir = TempDir::new().unwrap();
    let config_dir = temp_dir.path().to_path_buf();
    
    // Set up environment variables
    if cfg!(windows) {
        env::set_var("APPDATA", config_dir.to_str().unwrap().to_owned());
    } else {
        env::set_var("HOME", config_dir.to_str().unwrap().to_owned());
    }
    env::remove_var("OBSIDIAN_VAULT");

    // Create a basic config
    let config = Config {
        vault: "/test/vault".to_string(),
        file_path_format: "test/{year}/{month}/{date}.md".to_string(),
        section_header: "## Test".to_string(),
        list_type: ListType::Bullet,
        template_path: None,
        locale: None,
        time_format: TimeFormat::Hour24,
    };

    (config_dir, config)
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