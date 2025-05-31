use std::fs::{create_dir_all, write};
use std::process::Command;
use chrono::Local;
use crate::config::Config;
use crate::utils::get_log_path_for_date;

pub fn edit_today_log(config: &Config) {
    let today = Local::now().date_naive();
    let file_path = get_log_path_for_date(today, config);
    create_dir_all(file_path.parent().unwrap()).expect("Kunne ikke opprette katalogstruktur");

    // Sørg for at filen eksisterer
    if !file_path.exists() {
        write(&file_path, "").expect("Kunne ikke opprette tom loggfil");
    }

    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());
    let status = Command::new(editor)
        .arg(&file_path)
        .status()
        .expect("Kunne ikke starte editor");

    if !status.success() {
        eprintln!("Editor avsluttet med feil");
    }
}

