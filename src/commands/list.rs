use chrono::{Duration, Local, NaiveDate};
use std::fs::read_to_string;
use crate::config::Config;
use crate::utils::{get_log_path_for_date, extract_log_entries};

pub fn list_log_for_day(days_ago: i64, config: &Config) {
    let target_date = Local::now().date_naive() - Duration::days(days_ago);
    list_log_for_date(target_date, config);
}

pub fn list_relative_day(args: &mut impl Iterator<Item = String>, config: &Config) {
    let b_days: i64 = args.next().unwrap_or_else(|| {
        eprintln!("Error: -b needs a numeric argument (eg -b 1 for yesterday)");
        std::process::exit(1);
    }).parse().unwrap_or_else(|_| {
        eprintln!("Error: -b must be numeric argument (eg -b 1 for yesterday)");
        std::process::exit(1);
    });

    list_log_for_day(b_days, config);
}

fn list_log_for_date(date: NaiveDate, config: &Config) {
    let file_path = get_log_path_for_date(date, config);
    let date_str = date.to_string();

    let content = match read_to_string(&file_path) {
        Ok(c) => c,
        Err(_) => {
            println!("No log/s found for {}", date_str);
            return;
        }
    };

    let (_, _, entries, _) = extract_log_entries(&content, &config.section_header,&config.list_type);

    if entries.is_empty() {
        println!("No log-section ({} ) found for {}", config.section_header, date_str);
    } else {
        println!("{} Log/s for {}:", config.section_header, date_str);
        for entry in entries {
            println!("{}", entry);
        }
    }
}

