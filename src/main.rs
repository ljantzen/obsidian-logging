use std::env;
use std::fs::{self, read_to_string, write};
use std::path::PathBuf;
use chrono::{Local, NaiveDate, NaiveTime, Duration, Datelike, Timelike};

fn main() {
    let mut args = env::args().skip(1);
    let first_arg = args.next();

    match first_arg.as_deref() {
        Some("-l") | Some("--list") => list_log_for_day(0),
        Some("-n") => {
            let n_days: i64 = args.next().unwrap_or_else(|| {
                eprintln!("Feil: -n krever et tall (f.eks. -n 1 for i går)");
                std::process::exit(1);
            }).parse().unwrap_or_else(|_| {
                eprintln!("Feil: -n må etterfølges av et heltall");
                std::process::exit(1);
            });

            list_log_for_day(n_days);
        }
        Some("-t") | Some("--time") => handle_log_with_optional_time(args),
        Some(other) => {
            let rest = std::iter::once(other.to_string()).chain(args).collect();
            handle_log_with_sentence(rest, None);
        }
        None => {
            eprintln!("Bruk: logg [-t hh:mm] <setning> | -l | -n <dager>");
            std::process::exit(1);
        }
    }
}

fn handle_log_with_optional_time(mut args: impl Iterator<Item = String>) {
    let time_str = args.next().unwrap_or_else(|| {
        eprintln!("Feil: -t/--time krever et argument i format hh:mm");
        std::process::exit(1);
    });

    let time_override = Some(NaiveTime::parse_from_str(&time_str, "%H:%M").unwrap_or_else(|_| {
        eprintln!("Feil: ugyldig klokkeslett '{}'. Bruk format hh:mm.", time_str);
        std::process::exit(1);
    }));

    let sentence_parts: Vec<String> = args.collect();
    if sentence_parts.is_empty() {
        eprintln!("Feil: Du må oppgi en setning.");
        std::process::exit(1);
    }

    handle_log_with_sentence(sentence_parts, time_override);
}

fn handle_log_with_sentence(sentence_parts: Vec<String>, time_override: Option<NaiveTime>) {
    let sentence = sentence_parts.join(" ");
    let now = Local::now();
    let date = now.date_naive();
    let time = time_override.unwrap_or_else(|| NaiveTime::from_hms_opt(now.hour(), now.minute(), 0).unwrap());
    let time_str = format!("{:02}:{:02}", time.hour(), time.minute());

    let file_path = get_log_path_for_date(date);
    fs::create_dir_all(file_path.parent().unwrap()).expect("Kunne ikke opprette katalogstruktur");

    let mut content = read_to_string(&file_path).unwrap_or_default();
    if !content.contains("## 🕗") {
        content.push_str("\n## 🕗\n\n");
    }

    let (before_log, after_log, mut entries) = extract_log_entries(&content);
    let new_entry = format!("* {} {}", time_str, sentence);
    entries.push(new_entry);
    entries.sort_by_key(|entry| entry[2..7].to_string());

    let new_content = format!(
        "{}## 🕗\n\n{}\n{}",
        before_log,
        entries.join("\n"),
        after_log
    );

    write(&file_path, new_content.trim_end().to_string() + "\n").expect("Kunne ikke skrive tilbake til fil");
    println!("Logg lagret");
}

fn list_log_for_day(days_ago: i64) {
    let target_date = Local::now().date_naive() - Duration::days(days_ago);
    let file_path = get_log_path_for_date(target_date);
    let date_str = target_date.to_string();

    let content = match read_to_string(&file_path) {
        Ok(c) => c,
        Err(_) => {
            println!("Ingen logg funnet for {}", date_str);
            return;
        }
    };

    let (_, _, entries) = extract_log_entries(&content);
    if entries.is_empty() {
        println!("Ingen loggseksjon (## 🕗) funnet for {}", date_str);
    } else {
        println!("🕗 Logg for {}:", date_str);
        for entry in entries {
            println!("{}", entry);
        }
    }
}

fn get_log_path_for_date(date: NaiveDate) -> PathBuf {
    let vault_dir = env::var("OBSIDIAN_VAULT").expect("Miljøvariabelen OBSIDIAN_VAULT er ikke satt");
    let mut path = PathBuf::from(vault_dir);
    path.push("10-Journal");
    path.push(format!("{:04}", date.year()));
    path.push(format!("{:02}", date.month()));
    path.push(format!("{}.md", date));
    path
}

fn extract_log_entries(content: &str) -> (String, String, Vec<String>) {
    let lines: Vec<&str> = content.lines().collect();
    if let Some(start) = lines.iter().position(|line| line.trim() == "## 🕗") {
        let mut i = start + 1;
        let mut entries = Vec::new();

        while i < lines.len() {
            let line = lines[i].trim();
            if line.starts_with("* ") {
                entries.push(line.to_string());
            } else if line.starts_with("## ") {
                break;
            }
            i += 1;
        }

        let before = lines[..start].join("\n") + "\n";
        let after = lines[i..].join("\n");
        (before, after, entries)
    } else {
        (content.to_string(), String::new(), vec![])
    }
}

