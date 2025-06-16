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

#[derive(Debug, PartialEq, Clone)]
pub enum TimeFormat {
    Hour12,
    Hour24,
}

impl Serialize for TimeFormat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            TimeFormat::Hour12 => serializer.serialize_str("12"),
            TimeFormat::Hour24 => serializer.serialize_str("24"),
        }
    }
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

impl<'de> Deserialize<'de> for TimeFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "12" | "12h" | "12hour" => Ok(TimeFormat::Hour12),
            "24" | "24h" | "24hour" => Ok(TimeFormat::Hour24),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid time format '{}'. Expected '12' or '24' (case insensitive)",
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

impl FromStr for TimeFormat {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "12" | "12h" | "12hour" => Ok(TimeFormat::Hour12),
            "24" | "24h" | "24hour" => Ok(TimeFormat::Hour24),
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

impl ToString for TimeFormat {
    fn to_string(&self) -> String {
        match self {
            TimeFormat::Hour12 => "12".to_string(),
            TimeFormat::Hour24 => "24".to_string(),
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
    #[serde(default = "default_time_format")]
    pub time_format: TimeFormat,
    #[serde(default = "default_time_label")]
    pub time_label: String,
    #[serde(default = "default_event_label")]
    pub event_label: String,
}

fn default_time_format() -> TimeFormat {
    TimeFormat::Hour24
}

fn default_time_label() -> String {
    "Tidspunkt".to_string()
}

fn default_event_label() -> String {
    "Hendelse".to_string()
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
            time_format: TimeFormat::Hour24,
            time_label: default_time_label(),
            event_label: default_event_label(),
        }
    }
}

impl Config {
    pub fn with_list_type(&self, list_type: ListType) -> Self {
        let mut config = self.clone();
        config.list_type = list_type;
        config
    }

    pub fn with_time_format(&self, time_format: TimeFormat) -> Self {
        let mut config = self.clone();
        config.time_format = time_format;
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

pub fn initialize_config() -> Config {
    let config_dir = get_config_dir();
    let config_path = config_dir.join("obsidian-logging.yaml");

    // Try to read config file
    if let Ok(config_str) = fs::read_to_string(&config_path) {
        if let Ok(config) = serde_yaml::from_str(&config_str) {
            return config;
        }
    }

    // Fall back to default config
    Config::default()
}

