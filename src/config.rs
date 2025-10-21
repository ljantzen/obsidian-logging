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
        use serde::de::Visitor;
        use std::fmt;

        struct TimeFormatVisitor;

        impl<'de> Visitor<'de> for TimeFormatVisitor {
            type Value = TimeFormat;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a string or integer representing time format (12 or 24)")
            }

            fn visit_str<E>(self, value: &str) -> Result<TimeFormat, E>
            where
                E: serde::de::Error,
            {
                match value.to_lowercase().as_str() {
                    "12" | "12h" | "12hour" => Ok(TimeFormat::Hour12),
                    "24" | "24h" | "24hour" => Ok(TimeFormat::Hour24),
                    _ => Err(E::custom(format!(
                        "Invalid time format '{}'. Expected '12' or '24' (case insensitive)",
                        value
                    ))),
                }
            }

            fn visit_u64<E>(self, value: u64) -> Result<TimeFormat, E>
            where
                E: serde::de::Error,
            {
                match value {
                    12 => Ok(TimeFormat::Hour12),
                    24 => Ok(TimeFormat::Hour24),
                    _ => Err(E::custom(format!(
                        "Invalid time format '{}'. Expected 12 or 24",
                        value
                    ))),
                }
            }

            fn visit_i64<E>(self, value: i64) -> Result<TimeFormat, E>
            where
                E: serde::de::Error,
            {
                match value {
                    12 => Ok(TimeFormat::Hour12),
                    24 => Ok(TimeFormat::Hour24),
                    _ => Err(E::custom(format!(
                        "Invalid time format '{}'. Expected 12 or 24",
                        value
                    ))),
                }
            }
        }

        deserializer.deserialize_any(TimeFormatVisitor)
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

#[derive(Debug, Clone, Serialize)]
pub struct Config {
    pub vault: String,
    pub file_path_format: String,
    pub section_header: String,
    pub list_type: ListType,
    pub template_path: Option<String>,
    pub locale: Option<String>,
    pub time_format: TimeFormat,
    pub time_label: String,
    pub event_label: String,
    pub category_headers: std::collections::HashMap<String, String>,
    pub phrases: std::collections::HashMap<String, String>,
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

impl Config {
    /// Get the conjunction word based on the configured locale
    pub fn get_conjunction(&self) -> &'static str {
        match self.locale.as_deref() {
            Some("no") | Some("nb") | Some("nn") => "og",
            Some("da") => "og",
            Some("sv") => "och",
            Some("de") => "und",
            Some("fr") => "et",
            Some("es") => "y",
            Some("it") => "e",
            Some("pt") => "e",
            Some("ru") => "и",
            Some("ja") => "と",
            Some("ko") => "와",
            Some("zh") => "和",
            _ => "and", // Default to English
        }
    }
}

impl<'de> Deserialize<'de> for Config {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, MapAccess, Visitor};
        use std::fmt;

        struct ConfigVisitor;

        impl<'de> Visitor<'de> for ConfigVisitor {
            type Value = Config;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a YAML configuration object")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Config, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut vault = None;
                let mut file_path_format = None;
                let mut section_header = None;
                let mut list_type = None;
                let mut template_path = None;
                let mut locale = None;
                let mut time_format = None;
                let mut time_label = None;
                let mut event_label = None;
                let mut category_headers = std::collections::HashMap::new();
                let mut phrases = std::collections::HashMap::new();

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "vault" => {
                            if vault.is_some() {
                                return Err(de::Error::duplicate_field("vault"));
                            }
                            vault = Some(map.next_value()?);
                        }
                        "file_path_format" => {
                            if file_path_format.is_some() {
                                return Err(de::Error::duplicate_field("file_path_format"));
                            }
                            file_path_format = Some(map.next_value()?);
                        }
                        "section_header" => {
                            if section_header.is_some() {
                                return Err(de::Error::duplicate_field("section_header"));
                            }
                            section_header = Some(map.next_value()?);
                        }
                        "list_type" => {
                            if list_type.is_some() {
                                return Err(de::Error::duplicate_field("list_type"));
                            }
                            list_type = Some(map.next_value()?);
                        }
                        "template_path" => {
                            if template_path.is_some() {
                                return Err(de::Error::duplicate_field("template_path"));
                            }
                            template_path = Some(map.next_value()?);
                        }
                        "locale" => {
                            if locale.is_some() {
                                return Err(de::Error::duplicate_field("locale"));
                            }
                            locale = Some(map.next_value()?);
                        }
                        "time_format" => {
                            if time_format.is_some() {
                                return Err(de::Error::duplicate_field("time_format"));
                            }
                            time_format = Some(map.next_value()?);
                        }
                        "time_label" => {
                            if time_label.is_some() {
                                return Err(de::Error::duplicate_field("time_label"));
                            }
                            time_label = Some(map.next_value()?);
                        }
                        "event_label" => {
                            if event_label.is_some() {
                                return Err(de::Error::duplicate_field("event_label"));
                            }
                            event_label = Some(map.next_value()?);
                        }
                        "phrases" => {
                            let phrases_map: std::collections::HashMap<String, String> = map.next_value()?;
                            phrases = phrases_map;
                        }
                        _ => {
                            // Check if this is a category header (starts with "section_header_")
                            if key.starts_with("section_header_") {
                                let value: String = map.next_value()?;
                                category_headers.insert(key, value);
                            } else {
                                // Skip unknown fields
                                let _: serde_yaml::Value = map.next_value()?;
                            }
                        }
                    }
                }

                Ok(Config {
                    vault: vault.unwrap_or_default(),
                    file_path_format: file_path_format.unwrap_or_else(|| {
                        if cfg!(windows) {
                            "10-Journal\\{year}\\{month}\\{date}.md".to_string()
                        } else {
                            "10-Journal/{year}/{month}/{date}.md".to_string()
                        }
                    }),
                    section_header: section_header.unwrap_or_else(|| "## 🕗".to_string()),
                    list_type: list_type.unwrap_or(ListType::Bullet),
                    template_path,
                    locale,
                    time_format: time_format.unwrap_or_else(default_time_format),
                    time_label: time_label.unwrap_or_else(default_time_label),
                    event_label: event_label.unwrap_or_else(default_event_label),
                    category_headers,
                    phrases,
                })
            }
        }

        deserializer.deserialize_map(ConfigVisitor)
    }
}

impl Default for Config {
    fn default() -> Self {
        let vault_dir = env::var("OBSIDIAN_VAULT_DIR").unwrap_or_else(|_| "".to_string());
        
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
            category_headers: std::collections::HashMap::new(),
            phrases: std::collections::HashMap::new(),
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

    /// Get the section header for a specific category
    /// Returns the default section_header if no category-specific header is found
    pub fn get_section_header_for_category(&self, category: Option<&str>) -> &str {
        if let Some(cat) = category {
            let key = format!("section_header_{}", cat);
            self.category_headers.get(&key).map(|s| s.as_str()).unwrap_or(&self.section_header)
        } else {
            &self.section_header
        }
    }

    pub fn initialize() -> Config {
        let config_dir = get_config_dir();
        let config_path = config_dir.join("obsidian-logging.yaml");

        // Try to read config file
        let mut config = if let Ok(config_str) = fs::read_to_string(&config_path) {
            if let Ok(config) = serde_yaml::from_str(&config_str) {
                config
            } else {
                Config::default()
            }
        } else {
            Config::default()
        };

        // Override vault setting with environment variable if set
        if let Ok(vault_dir) = env::var("OBSIDIAN_VAULT_DIR") {
            config.vault = vault_dir;
        }

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

