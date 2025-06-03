use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::env;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub vault: String,
    pub file_path_format: String,
    pub section_header: String,
}

impl Default for Config {
    fn default() -> Self {
        let vault_dir = env::var("OBSIDIAN_VAULT").unwrap_or_else(|_| "".to_string());
        Config {
            vault: vault_dir,
            file_path_format: "10-Journal/{year}/{month}/{date}.md".to_string(),
            section_header: "## 🕗".to_string(),
        }
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

