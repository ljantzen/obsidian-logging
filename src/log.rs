use std::fs::{read_to_string, write, create_dir_all};
use crate::utils::extract_log_entries;
use crate::config::Config;

/// Leser innholdet i loggfilen for en gitt dato (basert på config).
pub fn read_log_for_date(date: chrono::NaiveDate, config: &Config) -> Option<String> {
    let path = crate::utils::get_log_path_for_date(date, config);
    match read_to_string(&path) {
        Ok(content) => Some(content),
        Err(_) => None,
    }
}

/// Skriver nytt innhold til loggfil for gitt dato.
/// Lager nødvendige kataloger om de ikke finnes.
pub fn write_log_for_date(date: chrono::NaiveDate, config: &Config, content: &str) -> std::io::Result<()> {
    let path = crate::utils::get_log_path_for_date(date, config);
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    write(path, content)
}

/// Legger til en loggoppføring i riktig seksjon, sortert kronologisk.
/// Returnerer Result for enkel feilbehandling.
pub fn add_log_entry(date: chrono::NaiveDate, time_str: &str, sentence: &str, config: &Config) -> Result<(), String> {
    let mut content = read_log_for_date(date, config).unwrap_or_default();

    if !content.contains(&config.layout.section_header) {
        content.push_str("\n");
        content.push_str(&config.layout.section_header);
        content.push_str("\n\n");
    }

    let (before, after, mut entries) = extract_log_entries(&content, &config.layout.section_header);
    let new_entry = format!("* {} {}", time_str, sentence);
    entries.push(new_entry);

    // Sorter loggene kronologisk etter tid (forutsatt hh:mm format i posisjon 2..7)
    entries.sort_by_key(|entry| entry[2..7].to_string());

    let new_content = format!("{}{}\
\n\n{}\n{}", before, config.layout.section_header, entries.join("\n"), after);

    write_log_for_date(date, config, &new_content)
        .map_err(|e| format!("Kunne ikke skrive til fil: {}", e))?;

    Ok(())
}

