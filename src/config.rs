use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use std::env;

#[derive(Debug, Deserialize)]
pub struct LayoutConfig {
    pub file_path_format: String,
    pub section_header: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub layout: LayoutConfig,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        LayoutConfig {
            file_path_format: "10-Journal/{year}/{month}/{date}.md".to_string(),
            section_header: "## 🕗".to_string(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            layout: LayoutConfig::default(),
        }
    }
}

pub fn load_config() -> Config {
    let home_dir = env::var("HOME").expect("HOME environment variable not set");
    let config_path: PathBuf = [home_dir.as_str(), ".config/olog/config.yaml"].iter().collect();

    if config_path.exists() {
        let content = fs::read_to_string(config_path).expect("Kunne ikke lese konfigurasjonsfil");
        serde_yaml::from_str(&content).unwrap_or_else(|e| {
            eprintln!("Feil i konfigurasjonsfil: {}", e);
            std::process::exit(1);
        })
    } else {
        Config::default()
    }
}

