use std::fs::{create_dir_all, write};
use std::process::Command;
use chrono::Local;
use crate::config::Config;
use crate::utils::get_log_path_for_date;

pub fn edit_today_log(config: &Config) {
    let today = Local::now().date_naive();
    let file_path = get_log_path_for_date(today, config);
    create_dir_all(file_path.parent().unwrap()).expect("Couldn't create parent directory");

    // Make sure the file exists
    if !file_path.exists() {
        write(&file_path, "").expect("Could not create empty log file");
    }

    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());
    let status = Command::new(editor)
        .arg(&file_path)
        .status()
        .expect("Failed to start editor");

    if !status.success() {
        eprintln!("Editor exited with non-zero exit code");
    }
}

