use crate::config::Config;
use crate::template::get_template_content;
use crate::utils::get_log_path_for_date;
use chrono::{Duration, Local};
use std::fs::{create_dir_all, write};
use std::process::Command;

pub fn edit_log_for_day(relative_day: i64, config: &Config, silent: bool) {
    let date = Local::now().date_naive() - Duration::days(relative_day);
    let file_path = get_log_path_for_date(date, config);
    create_dir_all(file_path.parent().unwrap()).expect("Couldn't create parent directory");

    // Only create a new file if it's today or a future date
    if !file_path.exists() && relative_day <= 0 {
        let template_content = get_template_content(config);
        write(&file_path, template_content).expect("Could not create log file from template");
    }

    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());
    let status = Command::new(editor)
        .arg(&file_path)
        .status()
        .expect("Failed to start editor");

    if !status.success() && !silent {
        eprintln!("Editor exited with non-zero exit code");
    }
}
