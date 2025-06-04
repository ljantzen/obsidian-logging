use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::env;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum ListType {
    Bullet,
    Table
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

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub vault: String,
    pub file_path_format: String,
    pub section_header: String,
    pub list_type: ListType,
}

impl Default for Config {
    fn default() -> Self {
        let vault_dir = env::var("OBSIDIAN_VAULT").unwrap_or_else(|_| "".to_string());
        Config {
            vault: vault_dir,
            file_path_format: "10-Journal/{year}/{month}/{date}.md".to_string(),
            section_header: "## 🕗".to_string(),
            list_type : ListType::Bullet
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

pub fn load_config() -> Config {
    let home_dir = env::var("HOME").expect("HOME environment variable not set");
    let config_path: PathBuf = [home_dir.as_str(), ".config/olog/olog.yaml"].iter().collect();

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

