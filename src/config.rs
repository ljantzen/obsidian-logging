use std::path::PathBuf;
use std::fs;
use serde::{Serialize, Deserialize};
use std::str::FromStr;
use std::env;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum ListType {
    Bullet,
    Table,
}

impl<'de> Deserialize<'de> for ListType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "bullet" => Ok(ListType::Bullet),
            "table" => Ok(ListType::Table),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid list type '{}'. Expected 'bullet' or 'table' (case insensitive)",
                s
            ))),
        }
    }
}

impl FromStr for ListType {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "bullet" => Ok(ListType::Bullet),
            "table" => Ok(ListType::Table),
            _ => Err(()),
        }
    }
}

impl ToString for ListType {
    fn to_string(&self) -> String {
        match self {
            ListType::Bullet => "bullet".to_string(),
            ListType::Table => "table".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Config {
    pub vault: String,
    pub file_path_format: String,
    pub section_header: String,
    pub list_type: ListType,
    pub template_path: Option<String>,
    pub locale: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        let vault_dir = env::var("OBSIDIAN_VAULT").unwrap_or_else(|_| "".to_string());
        
        Config {
            vault: vault_dir,
            file_path_format: if cfg!(windows) {
                "10-Journal\\{year}\\{month}\\{date}.md".to_string()
            } else {
                "10-Journal/{year}/{month}/{date}.md".to_string()
            },
            section_header: "## 🕗".to_string(),
            list_type: ListType::Bullet,
            template_path: None,
            locale: None,
        }
    }
}

impl Config {
    pub fn with_list_type(&self, list_type: ListType) -> Self {
        let mut config = self.clone();
        config.list_type = list_type;
        config
    }
}

fn get_config_dir() -> PathBuf {
    if cfg!(windows) {
        // On Windows, use %APPDATA%\obsidian-logging
        let app_data = env::var("APPDATA").expect("APPDATA environment variable not set");
        PathBuf::from(app_data).join("obsidian-logging")
    } else {
        // On Unix, use ~/.config/obsidian-logging
        let home = env::var("HOME").expect("HOME environment variable not set");
        PathBuf::from(home).join(".config").join("obsidian-logging")
    }
}

fn expand_tilde(path: &str) -> String {
    if path.starts_with("~/") || path.starts_with("~\\") {
        let home = if cfg!(windows) {
            // On Windows, use USERPROFILE
            env::var("USERPROFILE").expect("USERPROFILE environment variable not set")
        } else {
            // On Unix, use HOME
            env::var("HOME").expect("HOME environment variable not set")
        };
        path.replacen("~", &home, 1)
    } else {
        path.to_string()
    }
}

pub fn initialize_config() -> Config {
    let config_dir = get_config_dir();
    let config_path = config_dir.join("obsidian-logging.yaml");

    // Create config directory if it doesn't exist
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).expect("Could not create config directory");
    }

    // If config file doesn't exist, prompt for configuration values
    if !config_path.exists() {
        println!("Welcome to obsidian-logging! Let's set up your configuration.");
        println!("\nPlease enter the path to your Obsidian vault:");
        println!("(This is the directory containing your Obsidian notes)");
        if cfg!(windows) {
            println!("Example: C:\\Users\\YourName\\Documents\\ObsidianVault");
        } else {
            println!("Example: ~/Documents/ObsidianVault");
        }
        
        let mut vault_dir = String::new();
        std::io::stdin().read_line(&mut vault_dir).expect("Failed to read input");
        let vault_dir = vault_dir.trim().to_string();
        let vault_dir = expand_tilde(&vault_dir);

        println!("\nEnter the file path format for your daily notes:");
        println!("(This is the path within your vault where daily notes are stored)");
        if cfg!(windows) {
            println!("Default: 10-Journal\\{{year}}\\{{month}}\\{{date}}.md");
        } else {
            println!("Default: 10-Journal/{{year}}/{{month}}/{{date}}.md");
        }
        println!("Available variables: {{year}}, {{month}}, {{date}}");
        
        let mut file_path_format = String::new();
        std::io::stdin().read_line(&mut file_path_format).expect("Failed to read input");
        let file_path_format = file_path_format.trim();
        let file_path_format = if file_path_format.is_empty() {
            if cfg!(windows) {
                "10-Journal\\{year}\\{month}\\{date}.md".to_string()
            } else {
                "10-Journal/{year}/{month}/{date}.md".to_string()
            }
        } else {
            file_path_format.to_string()
        };

        println!("\nEnter the section header for log entries:");
        println!("(This is the markdown header that marks where log entries should be added)");
        println!("Default: ## 🕗");
        
        let mut section_header = String::new();
        std::io::stdin().read_line(&mut section_header).expect("Failed to read input");
        let section_header = section_header.trim();
        let section_header = if section_header.is_empty() {
            "## 🕗".to_string()
        } else {
            section_header.to_string()
        };

        // Create default config
        let config = Config {
            vault: vault_dir,
            file_path_format,
            section_header,
            list_type: ListType::Bullet,
            template_path: None,
            locale: None,
        };

        // Save config to file
        let yaml = serde_yaml::to_string(&config).expect("Could not serialize config");
        fs::write(&config_path, yaml).expect("Could not write config file");

        config
    } else {
        load_config()
    }
}

pub fn load_config() -> Config {
    let config_path = get_config_dir().join("obsidian-logging.yaml");

    if config_path.exists() {
        let content = fs::read_to_string(config_path).expect("Could not read config file");
        serde_yaml::from_str(&content).unwrap_or_else(|e| {
            eprintln!("Error in configuration file: {}", e);
            std::process::exit(1);
        })
    } else {
        Config::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;
    use std::env;
    use std::sync::{Once, Mutex};
    use lazy_static::lazy_static;

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
                env::remove_var("OBSIDIAN_VAULT");
            }
        });

        config_dir
    }

    #[test]
    fn test_expand_tilde() {
        let home_dir = setup_test_env();
        
        let test_path = "~/test/path";
        let expanded = expand_tilde(test_path);
        let expected = home_dir.join("test/path").to_string_lossy().into_owned();
        
        assert_eq!(expanded, expected);
    }

    #[test]
    fn test_get_config_dir() {
        let config_dir = setup_test_env();
        
        let result = get_config_dir();
        let expected = if cfg!(windows) {
            config_dir.join("obsidian-logging")
        } else {
            config_dir.join(".config").join("obsidian-logging")
        };
        
        assert_eq!(result.to_string_lossy(), expected.to_string_lossy());
    }

    #[test]
    fn test_load_config_default() {
        let _config_dir = setup_test_env();
        
        // Test loading when config doesn't exist
        let config = Config::default();
        assert_eq!(config.vault, "");
        assert_eq!(config.list_type, ListType::Bullet);
    }

    #[test]
    fn test_load_config_existing() {
        let config_dir = setup_test_env();
        
        // Create config directory and file
        let olog_dir = if cfg!(windows) {
            config_dir.join("obsidian-logging")
        } else {
            config_dir.join(".config").join("obsidian-logging")
        };
        fs::create_dir_all(&olog_dir).unwrap();
        
        let mut test_path = PathBuf::from("/test");
        test_path.push("vault");
        let vault_path = test_path.to_str().unwrap().to_string();

        let config = Config {
            vault: vault_path.clone(),
            file_path_format: "test/{year}/{month}/{date}.md".to_string(),
            section_header: "## Test".to_string(),
            list_type: ListType::Table,
            template_path: None,
            locale: None,
        };
        
        let yaml = serde_yaml::to_string(&config).unwrap();
        fs::write(olog_dir.join("obsidian-logging.yaml"), yaml).unwrap();
        
        // Test loading existing config
        let loaded_config = load_config();
        assert_eq!(loaded_config.vault, vault_path);
        assert_eq!(loaded_config.list_type, ListType::Table);
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
        let mut test_path = PathBuf::from("/test");
        test_path.push("vault");
        let vault_path = test_path.to_str().unwrap().to_string();

        let mut template_path = PathBuf::from("/test");
        template_path.push("template.md");
        let template_path_str = template_path.to_str().unwrap().to_string();

        let config = Config {
            vault: vault_path,
            file_path_format: "test/{year}/{month}/{date}.md".to_string(),
            section_header: "## Test".to_string(),
            list_type: ListType::Bullet,
            template_path: Some(template_path_str),
            locale: Some("en_US".to_string()),
        };
        
        let yaml = serde_yaml::to_string(&config).unwrap();
        let config_back: Config = serde_yaml::from_str(&yaml).unwrap();
        
        assert_eq!(config.vault, config_back.vault);
        assert_eq!(config.file_path_format, config_back.file_path_format);
        assert_eq!(config.section_header, config_back.section_header);
        assert_eq!(config.list_type, config_back.list_type);
        assert_eq!(config.template_path, config_back.template_path);
        assert_eq!(config.locale, config_back.locale);
    }
}

