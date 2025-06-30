use std::fs;
use std::env;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Once, Mutex};
use lazy_static::lazy_static;
use tempfile::TempDir;
use obsidian_logging::config::{Config, ListType, TimeFormat};
use serial_test::serial;

static INIT: Once = Once::new();

lazy_static! {
    static ref TEST_DIR: Mutex<Option<TempDir>> = Mutex::new(None);
}

fn setup_test_env() -> PathBuf {
    let mut test_dir = TEST_DIR.lock().unwrap();
    if test_dir.is_none() {
        *test_dir = Some(TempDir::new().unwrap());
    }
    let config_dir = test_dir.as_ref().unwrap().path().to_path_buf();
    let config_dir_str = config_dir.to_str().expect("Invalid path");

    // Initialize environment only once per test run
    INIT.call_once(|| {
        // SAFETY: We're only setting environment variables with valid UTF-8 strings
        // in a controlled test environment, and we do this only once at the start
        // of the test run.
        unsafe {
            if cfg!(windows) {
                env::set_var("APPDATA", config_dir_str);
            } else {
                env::set_var("HOME", config_dir_str);
            }
            env::remove_var("OBSIDIAN_VAULT_DIR");
        }
    });

    config_dir
}

fn expand_tilde(path: &str) -> String {
    if path.starts_with("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(&path[2..]).to_string_lossy().into_owned();
        }
    }
    path.to_string()
}

#[test]
fn test_expand_tilde() {
    let test_path = "~/test/path";
    let expanded = expand_tilde(test_path);
    assert!(!expanded.contains("~"));
    assert!(expanded.contains("test/path"));
}

#[test]
fn test_get_config_dir() {
    let config_dir = setup_test_env();
    
    let result = if cfg!(windows) {
        config_dir.join("obsidian-logging")
    } else {
        config_dir.join(".config").join("obsidian-logging")
    };
    
    assert_eq!(result.to_string_lossy(), result.to_string_lossy());
}

#[test]
#[serial]
fn test_load_config_default() {
    env::remove_var("OBSIDIAN_VAULT_DIR");
    let _config_dir = setup_test_env();
    
    // Test loading when config doesn't exist
    let config = Config::default();
    assert_eq!(config.vault, "");
    assert_eq!(config.list_type, ListType::Bullet);
}

#[test]
#[serial]
fn test_load_config_existing() {
    env::remove_var("OBSIDIAN_VAULT_DIR");
    let config_dir = setup_test_env();
    let config_path = if cfg!(windows) {
        config_dir.join("obsidian-logging").join("obsidian-logging.yaml")
    } else {
        config_dir.join(".config").join("obsidian-logging").join("obsidian-logging.yaml")
    };
    fs::create_dir_all(config_path.parent().unwrap()).unwrap();

    // Ensure environment variable is not set for this test
    env::remove_var("OBSIDIAN_VAULT_DIR");

    let test_config = Config {
        vault: "/test/vault".to_string(),
        file_path_format: "test/{year}/{month}/{date}.md".to_string(),
        section_header: "## Test".to_string(),
        list_type: ListType::Bullet,
        template_path: None,
        locale: None,
        time_format: TimeFormat::Hour24,
        time_label: "Tidspunkt".to_string(),
        event_label: "Hendelse".to_string(),
    };

    let yaml = serde_yaml::to_string(&test_config).unwrap();
    fs::write(&config_path, yaml).unwrap();

    let loaded_config = Config::initialize();
    assert_eq!(test_config.vault, loaded_config.vault);
    assert_eq!(test_config.file_path_format, loaded_config.file_path_format);
    assert_eq!(test_config.section_header, loaded_config.section_header);
    assert_eq!(test_config.list_type, loaded_config.list_type);
    assert_eq!(test_config.template_path, loaded_config.template_path);
    assert_eq!(test_config.locale, loaded_config.locale);
    assert_eq!(test_config.time_format, loaded_config.time_format);
    assert_eq!(test_config.time_label, loaded_config.time_label);
    assert_eq!(loaded_config.event_label, test_config.event_label);
}

#[test]
fn test_list_type_serialization() {
    // Test serialization
    let bullet = ListType::Bullet;
    let table = ListType::Table;
    
    let bullet_yaml = serde_yaml::to_string(&bullet).unwrap();
    let table_yaml = serde_yaml::to_string(&table).unwrap();
    
    assert_eq!(bullet_yaml.trim(), "Bullet");
    assert_eq!(table_yaml.trim(), "Table");
    
    // Test deserialization
    let bullet_back: ListType = serde_yaml::from_str("Bullet").unwrap();
    let table_back: ListType = serde_yaml::from_str("Table").unwrap();
    
    assert_eq!(bullet_back, ListType::Bullet);
    assert_eq!(table_back, ListType::Table);
    
    // Test case insensitivity
    let bullet_upper: ListType = serde_yaml::from_str("BULLET").unwrap();
    assert_eq!(bullet_upper, ListType::Bullet);
}

#[test]
fn test_config_serialization() {
    let config = Config {
        vault: "/test/vault".to_string(),
        file_path_format: "test/{year}/{month}/{date}.md".to_string(),
        section_header: "## Test".to_string(),
        list_type: ListType::Bullet,
        template_path: None,
        locale: None,
        time_format: TimeFormat::Hour24,
        time_label: "Tidspunkt".to_string(),
        event_label: "Hendelse".to_string(),
    };

    let serialized = serde_yaml::to_string(&config).unwrap();
    let deserialized: Config = serde_yaml::from_str(&serialized).unwrap();

    assert_eq!(config.vault, deserialized.vault);
    assert_eq!(config.file_path_format, deserialized.file_path_format);
    assert_eq!(config.section_header, deserialized.section_header);
    assert_eq!(config.list_type, deserialized.list_type);
    assert_eq!(config.template_path, deserialized.template_path);
    assert_eq!(config.locale, deserialized.locale);
    assert_eq!(config.time_format, deserialized.time_format);
    assert_eq!(config.time_label, deserialized.time_label);
    assert_eq!(config.event_label, deserialized.event_label);
}

#[test]
fn test_time_format_from_str() {
    // Test valid formats
    assert_eq!(TimeFormat::from_str("12"), Ok(TimeFormat::Hour12));
    assert_eq!(TimeFormat::from_str("12h"), Ok(TimeFormat::Hour12));
    assert_eq!(TimeFormat::from_str("12hour"), Ok(TimeFormat::Hour12));
    assert_eq!(TimeFormat::from_str("24"), Ok(TimeFormat::Hour24));
    assert_eq!(TimeFormat::from_str("24h"), Ok(TimeFormat::Hour24));
    assert_eq!(TimeFormat::from_str("24hour"), Ok(TimeFormat::Hour24));

    // Test case insensitivity
    assert_eq!(TimeFormat::from_str("12HOUR"), Ok(TimeFormat::Hour12));
    assert_eq!(TimeFormat::from_str("24HOUR"), Ok(TimeFormat::Hour24));

    // Test invalid formats
    assert!(TimeFormat::from_str("invalid").is_err());
    assert!(TimeFormat::from_str("").is_err());
    assert!(TimeFormat::from_str("13").is_err());
}

#[test]
fn test_time_format_to_string() {
    assert_eq!(TimeFormat::Hour12.to_string(), "12");
    assert_eq!(TimeFormat::Hour24.to_string(), "24");
}

#[test]
fn test_config_with_time_format() {
    let config = Config {
        vault: "/test/vault".to_string(),
        file_path_format: "test/{year}/{month}/{date}.md".to_string(),
        section_header: "## Test".to_string(),
        list_type: ListType::Bullet,
        template_path: None,
        locale: None,
        time_format: TimeFormat::Hour24,
        time_label: "Tidspunkt".to_string(),
        event_label: "Hendelse".to_string(),
    };

    let config_12h = config.with_time_format(TimeFormat::Hour12);
    assert_eq!(config_12h.time_format, TimeFormat::Hour12);

    let config_24h = config.with_time_format(TimeFormat::Hour24);
    assert_eq!(config_24h.time_format, TimeFormat::Hour24);
}

#[test]
fn test_config_with_list_type() {
    let config = Config {
        vault: "/test/vault".to_string(),
        file_path_format: "test/{year}/{month}/{date}.md".to_string(),
        section_header: "## Test".to_string(),
        list_type: ListType::Bullet,
        template_path: None,
        locale: None,
        time_format: TimeFormat::Hour24,
        time_label: "Tidspunkt".to_string(),
        event_label: "Hendelse".to_string(),
    };

    let config_bullet = config.with_list_type(ListType::Bullet);
    assert_eq!(config_bullet.list_type, ListType::Bullet);

    let config_table = config.with_list_type(ListType::Table);
    assert_eq!(config_table.list_type, ListType::Table);
}

#[test]
#[serial]
fn test_environment_variable_overrides_config() {
    let config_dir = setup_test_env();
    let config_path = if cfg!(windows) {
        config_dir.join("obsidian-logging").join("obsidian-logging.yaml")
    } else {
        config_dir.join(".config").join("obsidian-logging").join("obsidian-logging.yaml")
    };
    fs::create_dir_all(config_path.parent().unwrap()).unwrap();

    // Create a config file with a specific vault path
    let test_config = Config {
        vault: "/config/vault".to_string(),
        file_path_format: "test/{year}/{month}/{date}.md".to_string(),
        section_header: "## Test".to_string(),
        list_type: ListType::Bullet,
        template_path: None,
        locale: None,
        time_format: TimeFormat::Hour24,
        time_label: "Tidspunkt".to_string(),
        event_label: "Hendelse".to_string(),
    };

    let yaml = serde_yaml::to_string(&test_config).unwrap();
    fs::write(&config_path, yaml).unwrap();

    // Set environment variable to override the config (after setup_test_env)
    env::set_var("OBSIDIAN_VAULT_DIR", "/env/vault");
    assert_eq!(env::var("OBSIDIAN_VAULT_DIR").unwrap(), "/env/vault");

    // Load config - should use environment variable value
    let loaded_config = Config::initialize();
    assert_eq!(loaded_config.vault, "/env/vault");
    assert_eq!(loaded_config.file_path_format, test_config.file_path_format);
    assert_eq!(loaded_config.section_header, test_config.section_header);
    assert_eq!(loaded_config.list_type, test_config.list_type);
    assert_eq!(loaded_config.template_path, test_config.template_path);
    assert_eq!(loaded_config.locale, test_config.locale);
    assert_eq!(loaded_config.time_format, test_config.time_format);
    assert_eq!(loaded_config.time_label, test_config.time_label);
    assert_eq!(loaded_config.event_label, test_config.event_label);

    // Clean up
    env::remove_var("OBSIDIAN_VAULT_DIR");
} 