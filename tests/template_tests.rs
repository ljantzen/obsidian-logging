use chrono::{Duration, Local};
use std::fs;
use tempfile::TempDir;
use obsidian_logging::config::{Config, ListType, TimeFormat};
use obsidian_logging::template::{TemplateData, process_template, get_template_content};
use regex::Regex;

#[test]
fn test_template_data_new() {
    let data = TemplateData::new(None);
    let now = Local::now();
    let today = now.date_naive();
    let yesterday = today - Duration::days(1);
    let tomorrow = today + Duration::days(1);
    
    assert_eq!(data.today, today.format("%Y-%m-%d").to_string());
    assert_eq!(data.yesterday, yesterday.format("%Y-%m-%d").to_string());
    assert_eq!(data.tomorrow, tomorrow.format("%Y-%m-%d").to_string());
    assert!(["monday", "tuesday", "wednesday", "thursday", "friday", "saturday", "sunday"]
        .contains(&data.weekday.as_str()));
    
    // Verify created timestamp format
    let timestamp_pattern = Regex::new(r"^\d{4}-\d{2}-\d{2} \d{2}:\d{2}$").unwrap();
    assert!(timestamp_pattern.is_match(&data.created));
}

#[test]
fn test_template_data_with_locale() {
    let data = TemplateData::new(Some("nb_NO"));
    let now = Local::now();
    let today = now.date_naive();
    let yesterday = today - Duration::days(1);
    let tomorrow = today + Duration::days(1);
    
    assert_eq!(data.today, today.format("%Y-%m-%d").to_string());
    assert_eq!(data.yesterday, yesterday.format("%Y-%m-%d").to_string());
    assert_eq!(data.tomorrow, tomorrow.format("%Y-%m-%d").to_string());
    assert!(["mandag", "tirsdag", "onsdag", "torsdag", "fredag", "lørdag", "søndag"]
        .contains(&data.weekday.as_str()));
    
    // Verify created timestamp format
    let timestamp_pattern = Regex::new(r"^\d{4}-\d{2}-\d{2} \d{2}:\d{2}$").unwrap();
    assert!(timestamp_pattern.is_match(&data.created));
}

#[test]
fn test_process_template() {
    let temp_dir = tempfile::tempdir().unwrap();
    let template_path = temp_dir.path().join("template.md");
    
    let template_content = r#"# {{today}} ({{weekday}})
Yesterday: [[{{yesterday}}]]
Tomorrow: [[{{tomorrow}}]]
Created: {{created}}"#;
    
    fs::write(&template_path, template_content).unwrap();
    
    let data = TemplateData::new(None);
    let result = process_template(template_path.to_str().unwrap(), &data);
    
    assert!(result.contains(&data.today));
    assert!(result.contains(&data.weekday));
    assert!(result.contains(&data.yesterday));
    assert!(result.contains(&data.tomorrow));
    assert!(result.contains(&data.created));
}

#[test]
fn test_get_template_content_with_template() {
    let temp_dir = TempDir::new().unwrap();
    let config = Config {
        vault: temp_dir.path().to_str().unwrap().to_string(),
        file_path_format: "{date}.md".to_string(),
        section_header: "## Test".to_string(),
        list_type: ListType::Bullet,
        template_path: Some("non-existent-template.md".to_string()),
        locale: None,
        time_format: TimeFormat::Hour24,
        time_label: "Tidspunkt".to_string(),
        event_label: "Hendelse".to_string(),
    };

    let content = get_template_content(&config);
    assert_eq!(content, "## 🕗\n\n");
}

#[test]
fn test_get_template_content_no_template() {
    let temp_dir = TempDir::new().unwrap();
    let config = Config {
        vault: temp_dir.path().to_str().unwrap().to_string(),
        file_path_format: "{date}.md".to_string(),
        section_header: "## Test".to_string(),
        list_type: ListType::Bullet,
        template_path: None,
        locale: None,
        time_format: TimeFormat::Hour24,
        time_label: "Tidspunkt".to_string(),
        event_label: "Hendelse".to_string(),
    };

    let content = get_template_content(&config);
    assert_eq!(content, "## 🕗\n\n");
} 