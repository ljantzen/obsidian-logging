use std::env;
use std::fs::{self, read_to_string, write};
use std::path::PathBuf;
use chrono::{Local, NaiveTime, Duration, Datelike, Timelike};

fn main() {
    let mut args = env::args().skip(1);
    let first_arg = args.next();

    match first_arg.as_deref() {
        Some("-l") | Some("--list") => {
            list_log_for_day(0); // dagens logg
            return;
        }
        Some("-n") => {
            let n_str = args.next().unwrap_or_else(|| {
                eprintln!("Feil: -n krever et tall (f.eks. -n 1 for i går)");
                std::process::exit(1);
            });

            let n_days: i64 = n_str.parse().unwrap_or_else(|_| {
                eprintln!("Feil: -n må etterfølges av et heltall");
                std::process::exit(1);
            });

            list_log_for_day(n_days);
            return;
        }
        Some("-t") | Some("--time") => {
            handle_log_with_optional_time(Some(args));
        }
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

fn handle_log_with_optional_time(mut args: Option<impl Iterator<Item = String>>) {
    let args = args.as_mut().unwrap();
    let time_str = args.next().unwrap_or_else(|| {
        eprintln!("Feil: -t/--time krever et argument i format hh:mm");
        std::process::exit(1);
    });

    let time_override = Some(
        NaiveTime::parse_from_str(&time_str, "%H:%M").unwrap_or_else(|_| {
            eprintln!("Feil: ugyldig klokkeslett '{}'. Bruk format hh:mm.", time_str);
            std::process::exit(1);
        }),
    );

    let sentence_parts: Vec<String> = args.collect();
    if sentence_parts.is_empty() {
        eprintln!("Feil: Du må oppgi en setning.");
        std::process::exit(1);
    }

    handle_log_with_sentence(sentence_parts, time_override);
}

fn handle_log_with_sentence(sentence_parts: Vec<String>, time_override: Option<NaiveTime>) {
    let sentence = sentence_parts.join(" ");

    let vault_dir = env::var("OBSIDIAN_VAULT").expect("Miljøvariabelen OBSIDIAN_VAULT er ikke satt");

    let now = Local::now();
    let year = format!("{:04}", now.year());
    let month = format!("{:02}", now.month());
    let day = format!("{:02}", now.day());
    let date_str = format!("{}-{}-{}", year, month, day);

    let time = time_override.unwrap_or_else(|| NaiveTime::from_hms_opt(now.hour(), now.minute(), 0).unwrap());
    let time_str = format!("{:02}:{:02}", time.hour(), time.minute());

    let mut file_path = PathBuf::from(&vault_dir);
    file_path.push("10-Journal");
    file_path.push(&year);
    file_path.push(&month);
    fs::create_dir_all(&file_path).expect("Kunne ikke opprette katalogstruktur");
    file_path.push(format!("{}.md", date_str));

    let mut content = read_to_string(&file_path).unwrap_or_else(|_| String::new());
    if !content.contains("## 🕗") {
        content.push_str("\n## 🕗\n\n");
    }

    let lines: Vec<&str> = content.lines().collect();
    let log_index = lines.iter().position(|line| line.trim() == "## 🕗").unwrap();
    let mut log_entries: Vec<String> = Vec::new();

    let mut i = log_index + 1;
    while i < lines.len() {
        let line = lines[i].trim();
        if line.starts_with("* ") {
            log_entries.push(line.to_string());
        } else if line.starts_with("## ") {
            break;
        }
        i += 1;
    }

    let new_entry = format!("* {} {}", time_str, sentence);
    log_entries.push(new_entry);
    log_entries.sort_by_key(|entry| entry[2..7].to_string());

    let mut new_lines = lines[..=log_index].to_vec();
    new_lines.push("");
    new_lines.extend(log_entries.iter().map(|s| s.as_str()));
    while i < lines.len() {
        new_lines.push(lines[i]);
        i += 1;
    }

    let final_content = new_lines.join("\n") + "\n";
    write(&file_path, final_content).expect("Kunne ikke skrive tilbake til fil");

    println!("Logg lagret");
}

fn list_log_for_day(days_ago: i64) {
    let vault_dir = env::var("OBSIDIAN_VAULT").expect("Miljøvariabelen OBSIDIAN_VAULT er ikke satt");

    let target_date = Local::now().date_naive() - Duration::days(days_ago);
    let year = format!("{:04}", target_date.year());
    let month = format!("{:02}", target_date.month());
    let day = format!("{:02}", target_date.day());
    let date_str = format!("{}-{}-{}", year, month, day);

    let mut file_path = PathBuf::from(&vault_dir);
    file_path.push("10-Journal");
    file_path.push(&year);
    file_path.push(&month);
    file_path.push(format!("{}.md", date_str));

    let content = match read_to_string(&file_path) {
        Ok(c) => c,
        Err(_) => {
            println!("Ingen logg funnet for {}", date_str);
            return;
        }
    };

    let lines: Vec<&str> = content.lines().collect();
    if let Some(log_index) = lines.iter().position(|line| line.trim() == "## 🕗") {
        let mut i = log_index + 1;
        println!("🕗 Logg for {}:", date_str);
        while i < lines.len() {
            let line = lines[i].trim();
            if line.starts_with("* ") {
                println!("{}", line);
            } else if line.starts_with("## ") {
                break;
            }
            i += 1;
        }
    } else {
        println!("Ingen loggseksjon (## 🕗) funnet for {}", date_str);
    }
}

