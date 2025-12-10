use crate::config::Config;
use crate::utils::{extract_log_entries, get_log_path_for_date};
use chrono::{Duration, Local};
use std::fs::read_to_string;

pub fn list_log_for_day(
    relative_day: i64,
    config: &Config,
    silent: bool,
    include_header: bool,
    categories: &[String],
) {
    let date = Local::now().date_naive() - Duration::days(relative_day);
    let log_path = get_log_path_for_date(date, config);

    if !log_path.exists() {
        if !silent {
            println!("No log found for {}", date);
        }
        return;
    }

    let content = read_to_string(&log_path).unwrap_or_else(|_| {
        if !silent {
            println!("Error reading log file");
        }
        String::new()
    });

    // Handle different category scenarios
    if categories.is_empty() {
        // No categories specified - list default section only
        let section_header = config.get_section_header_for_category(None);
        let (_, _, entries, _) = extract_log_entries(
            &content,
            section_header,
            &config.list_type,
            config,
            include_header,
        );

        if entries.is_empty() {
            if !silent {
                println!("No entries found for {}", date);
            }
            return;
        }

        if !silent {
            println!("Log entries for {}:", date);
            for entry in entries {
                println!("{}", entry);
            }
        }
    } else if categories.len() == 1 && categories[0] == "all" {
        // Special case: list all categories
        list_all_categories(&content, config, silent, include_header, date);
    } else {
        // List specific categories
        list_specific_categories(&content, config, silent, include_header, date, categories);
    }
}

fn list_all_categories(
    content: &str,
    config: &Config,
    silent: bool,
    include_header: bool,
    date: chrono::NaiveDate,
) {
    if silent {
        return;
    }

    println!("Log entries for {} (all categories):", date);

    // List default section first
    let default_header = config.get_section_header_for_category(None);
    let (_, _, default_entries, _) = extract_log_entries(
        content,
        default_header,
        &config.list_type,
        config,
        include_header,
    );

    if !default_entries.is_empty() {
        println!("\n{}", default_header);
        for entry in default_entries {
            println!("{}", entry);
        }
    }

    // List all category sections
    for (key, header) in &config.category_headers {
        if key.starts_with("section_header_") {
            let (_, _, entries, _) =
                extract_log_entries(content, header, &config.list_type, config, include_header);

            if !entries.is_empty() {
                println!("\n{}", header);
                for entry in entries {
                    println!("{}", entry);
                }
            }
        }
    }
}

fn list_specific_categories(
    content: &str,
    config: &Config,
    silent: bool,
    include_header: bool,
    date: chrono::NaiveDate,
    categories: &[String],
) {
    if silent {
        return;
    }

    let category_list = categories.join(", ");
    println!("Log entries for {} (categories: {}):", date, category_list);

    let mut found_any = false;

    for category in categories {
        let section_header = config.get_section_header_for_category(Some(category));
        let (_, _, entries, _) = extract_log_entries(
            content,
            section_header,
            &config.list_type,
            config,
            include_header,
        );

        if !entries.is_empty() {
            found_any = true;
            println!("\n{}", section_header);
            for entry in entries {
                println!("{}", entry);
            }
        }
    }

    if !found_any {
        println!("No entries found for the specified categories.");
    }
}
