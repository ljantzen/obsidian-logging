use std::env;
use std::fs::{self, read_to_string, write};
use std::path::PathBuf;
use chrono::{Local, NaiveTime, Datelike, Timelike};

fn main() {
    // Hent setning fra CLI og valgfri --time hh:mm
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Bruk: logg [--time hh:mm] <setning>");
        std::process::exit(1);
    }

    let mut time_override: Option<NaiveTime> = None;
    let mut sentence_parts: Vec<String> = Vec::new();

    let mut args_iter = args.into_iter();
    while let Some(arg) = args_iter.next() {
        if arg == "--time" || arg == "-t"  {
            if let Some(time_str) = args_iter.next() {
                match NaiveTime::parse_from_str(&time_str, "%H:%M") {
                    Ok(t) => time_override = Some(t),
                    Err(_) => {
                        eprintln!("Feil: ugyldig klokkeslett '{}'. Bruk format hh:mm.", time_str);
                        std::process::exit(1);
                    }
                }
            } else {
                eprintln!("Feil: -t/--time krever et argument");
                std::process::exit(1);
            }
        } else {
            sentence_parts.push(arg);
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
            break; // slutt på loggseksjonen
        }
        i += 1;
    }

    // Legg til ny logglinje
    let new_entry = format!("* {} {}", time_str, sentence);
    log_entries.push(new_entry);

    // Sorter logglinjer etter tid
    log_entries.sort_by_key(|entry| {
        entry[2..7].to_string() // hh:mm
    });

    // Sett sammen ny fil
    let mut new_lines = lines[..=log_index].to_vec(); // inkludert ## 🕗
    new_lines.push(""); // tom linje etter overskriften
    new_lines.extend(log_entries.iter().map(|s| s.as_str()));

    // Finn og legg til resterende linjer etter loggseksjonen
    while i < lines.len() {
        new_lines.push(lines[i]);
        i += 1;
    }

    // Skriv fil på nytt
    let final_content = new_lines.join("\n") + "\n";
    write(&file_path, final_content).expect("Kunne ikke skrive tilbake til fil");

    println!("Log accepted");
}

