use std::env;
use std::fs::{self, read_to_string, write};
use std::path::PathBuf;
use chrono::{Local, NaiveTime, Datelike, Timelike};

fn main() {
    // Parse args
    let mut args = env::args().skip(1); // skip binary name
    let mut time_override: Option<NaiveTime> = None;

    // Check if first argument is -t or --time
    let first_arg = args.next();
    let mut sentence_parts: Vec<String>;

    match first_arg.as_deref() {
        Some("-t") | Some("--time") => {
            let time_str = args.next().unwrap_or_else(|| {
                eprintln!("Feil: -t/--time krever et argument i format hh:mm");
                std::process::exit(1);
            });

            time_override = Some(
                NaiveTime::parse_from_str(&time_str, "%H:%M").unwrap_or_else(|_| {
                    eprintln!("Feil: ugyldig klokkeslett '{}'. Bruk format hh:mm.", time_str);
                    std::process::exit(1);
                }),
            );

            sentence_parts = args.collect();
        }
        Some(other) => {
            sentence_parts = vec![other.to_string()];
            sentence_parts.extend(args);
        }
        None => {
            eprintln!("Bruk: olog [-t hh:mm] <setning>");
            std::process::exit(1);
        }
    }

    if sentence_parts.is_empty() {
        eprintln!("Feil: Du må oppgi en setning.");
        std::process::exit(1);
    }

    let sentence = sentence_parts.join(" ");

    // Hent OBSIDIAN_VAULT-sti
    let vault_dir = env::var("OBSIDIAN_VAULT").expect("Miljøvariabelen OBSIDIAN_VAULT er ikke satt");

    // Tid og dato
    let now = Local::now();
    let year = format!("{:04}", now.year());
    let month = format!("{:02}", now.month());
    let day = format!("{:02}", now.day());
    let date_str = format!("{}-{}-{}", year, month, day);

    let time = time_override.unwrap_or_else(|| NaiveTime::from_hms_opt(now.hour(), now.minute(), 0).unwrap());
    let time_str = format!("{:02}:{:02}", time.hour(), time.minute());

    // Filsti
    let mut file_path = PathBuf::from(&vault_dir);
    file_path.push("10-Journal");
    file_path.push(&year);
    file_path.push(&month);
    fs::create_dir_all(&file_path).expect("Kunne ikke opprette katalogstruktur");
    file_path.push(format!("{}.md", date_str));

    // Les fil eller lag ny
    let mut content = read_to_string(&file_path).unwrap_or_else(|_| String::new());
    if !content.contains("## 🕗") {
        content.push_str("\n## 🕗\n\n");
    }

    let lines: Vec<&str> = content.lines().collect();
    let log_index = lines.iter().position(|line| line.trim() == "## 🕗").unwrap();
    let mut log_entries: Vec<String> = Vec::new();

    // Samle eksisterende logglinjer etter ## 🕗
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

    // Legg til ny logglinje
    let new_entry = format!("* {} {}", time_str, sentence);
    log_entries.push(new_entry);

    // Sorter logglinjer etter tid
    log_entries.sort_by_key(|entry| entry[2..7].to_string()); // assumes format "* hh:mm ..."

    // Sett sammen ny fil
    let mut new_lines = lines[..=log_index].to_vec();
    new_lines.push("");
    new_lines.extend(log_entries.iter().map(|s| s.as_str()));

    // Legg til resten av linjene
    while i < lines.len() {
        new_lines.push(lines[i]);
        i += 1;
    }

    // Skriv fil
    let final_content = new_lines.join("\n") + "\n";
    write(&file_path, final_content).expect("Kunne ikke skrive tilbake til fil");

    println!("Log accepted");
}

