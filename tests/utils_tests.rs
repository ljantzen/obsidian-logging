use chrono::{NaiveDate, NaiveTime};
use obsidian_logging::config::{Config, ListType, TimeFormat};
use obsidian_logging::utils::{get_log_path_for_date, extract_log_entries, format_time, parse_time};
use std::path::PathBuf;

fn create_test_config() -> Config {
    Config {
        vault: "/test/vault".to_string(),
        file_path_format: "test/{year}/{month}/{date}.md".to_string(),
        section_header: "## Test".to_string(),
        list_type: ListType::Bullet,
        template_path: None,
        locale: None,
        time_format: TimeFormat::Hour24,
        time_label: "Tidspunkt".to_string(),
        event_label: "Hendelse".to_string(),
        category_headers: std::collections::HashMap::new(),
        phrases: std::collections::HashMap::new(),
    }
}

#[test]
fn test_get_log_path_for_date() {
    let config = create_test_config();
    let date = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
    
    let path = get_log_path_for_date(date, &config);
    let mut expected_path = PathBuf::from("/test/vault");
    expected_path.push("test");
    expected_path.push("2024");
    expected_path.push("03");
    expected_path.push("2024-03-15.md");
    
    assert_eq!(path, expected_path);
}

#[test]
fn test_extract_log_entries_bullet() {
    let content = r#"# Header
Some content

## Test
* 09:00 First entry
* 10:30 Second entry
* 11:15 Third entry

## Another section"#;

    let config = create_test_config();
    let (before, after, entries, found_type) = extract_log_entries(content, &config.section_header, &ListType::Bullet, &config, false);

    assert_eq!(before, "# Header\nSome content\n\n");
    assert_eq!(after, "## Another section");
    assert_eq!(entries, vec![
        "* 09:00 First entry",
        "* 10:30 Second entry",
        "* 11:15 Third entry"
    ]);
    assert_eq!(found_type, ListType::Bullet);
}

#[test]
fn test_extract_log_entries_table() {
    let content = r#"# Header
Some content

## Test
| Tidspunkt | Hendelse |
|-----------|----------|
| 09:00 | First entry |
| 10:30 | Second entry |
| 11:15 | Third entry |

## Another section"#;

    let config = create_test_config();
    let (before, after, entries, found_type) = extract_log_entries(content, &config.section_header, &ListType::Table, &config, false);

    assert_eq!(before, "# Header\nSome content\n\n");
    assert_eq!(after, "## Another section");
    assert_eq!(entries, vec![
        "| 09:00 | First entry |",
        "| 10:30 | Second entry |",
        "| 11:15 | Third entry |"
    ]);
    assert_eq!(found_type, ListType::Table);
}

#[test]
fn test_extract_log_entries_empty() {
    let content = r#"# Header
Some content

## Test

## Another section"#;

    let config = create_test_config();
    let (before, after, entries, found_type) = extract_log_entries(content, &config.section_header, &ListType::Bullet, &config, false);

    assert_eq!(before, "# Header\nSome content\n\n");
    assert_eq!(after, "## Another section");
    assert!(entries.is_empty());
    assert_eq!(found_type, ListType::Bullet);
}

#[test]
fn test_extract_log_entries_no_section() {
    let content = "# Header\nSome content\n";

    let config = create_test_config();
    let (before, after, entries, found_type) = extract_log_entries(content, &config.section_header, &ListType::Bullet, &config, false);

    assert_eq!(before, content);
    assert_eq!(after, "");
    assert!(entries.is_empty());
    assert_eq!(found_type, ListType::Bullet);
}

#[test]
fn test_extract_log_entries_convert_bullet_to_table() {
    let content = r#"## Test
* 09:00 First entry
* 10:30 Second entry"#;

    let config = create_test_config();
    let (_, _, entries, _) = extract_log_entries(content, &config.section_header, &ListType::Table, &config, true);

    // Should convert to table format with consistent column widths (with seconds added during reformatting)
    assert_eq!(entries[0], "| Tidspunkt | Hendelse     |");
    assert_eq!(entries[1], "|-----------|--------------|");
    assert_eq!(entries[2], "| 09:00:00  | First entry  |");
    assert_eq!(entries[3], "| 10:30:00  | Second entry |");
}

#[test]
fn test_extract_log_entries_convert_table_to_bullet() {
    let content = r#"## Test
| Tidspunkt | Hendelse |
|-----------|----------|
| 09:00 | First entry |
| 10:30 | Second entry |"#;

    let config = create_test_config();
    let (_, _, entries, _) = extract_log_entries(content, &config.section_header, &ListType::Bullet, &config, false);

    // Should convert to bullet format (with seconds added during reformatting)
    assert_eq!(entries[0], "- 09:00:00 First entry");
    assert_eq!(entries[1], "- 10:30:00 Second entry");
}

#[test]
fn test_extract_log_entries_table_format() {
    let content = r#"# Header
Some content

## Test
| Tidspunkt | Hendelse |
|-----------|----------|
| 09:00 | First entry |
| 10:30 | Second entry |
| 11:15 | Third entry |

## Another section"#;

    let config = create_test_config();
    let (before, after, entries, found_type) = extract_log_entries(content, &config.section_header, &ListType::Table, &config, false);

    assert_eq!(before, "# Header\nSome content\n\n");
    assert_eq!(after, "## Another section");
    assert_eq!(entries, vec![
        "| 09:00 | First entry |",
        "| 10:30 | Second entry |",
        "| 11:15 | Third entry |"
    ]);
    assert_eq!(found_type, ListType::Table);
}

#[test]
fn test_format_time_24h() {
    let time = NaiveTime::from_hms_opt(14, 30, 45).unwrap();
    let formatted = format_time(time, &TimeFormat::Hour24);
    assert_eq!(formatted, "14:30:45");
    
    // Test with zero seconds
    let time_zero = NaiveTime::from_hms_opt(14, 30, 0).unwrap();
    let formatted_zero = format_time(time_zero, &TimeFormat::Hour24);
    assert_eq!(formatted_zero, "14:30:00");
}

#[test]
fn test_format_time_12h() {
    let test_cases = vec![
        (0, 30, 0, "12:30:00 AM"),
        (1, 30, 0, "01:30:00 AM"),
        (11, 30, 0, "11:30:00 AM"),
        (12, 30, 0, "12:30:00 PM"),
        (13, 30, 0, "01:30:00 PM"),
        (23, 30, 0, "11:30:00 PM"),
        (14, 30, 45, "02:30:45 PM"),
    ];

    for (hour, minute, second, expected) in test_cases {
        let time = NaiveTime::from_hms_opt(hour, minute, second).unwrap();
        let formatted = format_time(time, &TimeFormat::Hour12);
        assert_eq!(formatted, expected);
    }
}

#[test]
fn test_parse_time() {
    // Test 24-hour format without seconds (should default to 00)
    assert_eq!(
        parse_time("14:30"),
        Some(NaiveTime::from_hms_opt(14, 30, 0).unwrap())
    );
    
    // Test 24-hour format with seconds
    assert_eq!(
        parse_time("14:30:45"),
        Some(NaiveTime::from_hms_opt(14, 30, 45).unwrap())
    );

    // Test 12-hour format with various formats (without seconds, should default to 00)
    let test_cases = vec![
        "02:30 PM",
        "02:30PM",
        "02:30 pm",
        "02:30pm",
        "2:30 PM",
        "2:30PM",
    ];

    for time_str in test_cases {
        assert_eq!(
            parse_time(time_str),
            Some(NaiveTime::from_hms_opt(14, 30, 0).unwrap()),
            "Failed to parse {}",
            time_str
        );
    }
    
    // Test 12-hour format with seconds
    let test_cases_with_seconds = vec![
        "02:30:45 PM",
        "02:30:45PM",
        "02:30:45 pm",
        "02:30:45pm",
        "2:30:45 PM",
        "2:30:45PM",
    ];

    for time_str in test_cases_with_seconds {
        assert_eq!(
            parse_time(time_str),
            Some(NaiveTime::from_hms_opt(14, 30, 45).unwrap()),
            "Failed to parse {}",
            time_str
        );
    }

    // Test invalid formats
    assert_eq!(parse_time("not a time"), None);
    assert_eq!(parse_time("25:00"), None);
    assert_eq!(parse_time("14:60"), None);
    assert_eq!(parse_time("14:30:60"), None); // Invalid seconds
    assert_eq!(parse_time("02:30 MP"), None);
}

#[test]
fn test_extract_log_entries_with_time_formats() {
    // Test with mixed 12/24 hour formats
    let content = r#"# Header
Some content

## Test
* 09:00 AM First entry
* 14:30 Second entry
* 02:15 PM Third entry

## Another section"#;

    let config = create_test_config();
    let (_, _, entries, _) = extract_log_entries(content, &config.section_header, &ListType::Bullet, &config, false);

    assert_eq!(entries, vec![
        "* 09:00 AM First entry",
        "* 14:30 Second entry",
        "* 02:15 PM Third entry"
    ]);
} 