use std::env;
use std::fs::{self, OpenOptions, read_to_string, write};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use chrono::{Local, Datelike, Timelike};

fn main() {
    // Hent setning fra CLI
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Bruk: logg <setning>");
        std::process::exit(1);
    }
    let sentence = args.join(" ");

    // Hent OBSIDIAN_VAULT-sti
    let vault_dir = env::var("OBSIDIAN_VAULT").expect("Miljøvariabelen OBSIDIAN_VAULT er ikke satt");

    // Tid og dato
    let now = Local::now();
    let year = format!("{:04}", now.year());
    let month = format!("{:02}", now.month());
    let day = format!("{:02}", now.day());
    let date_str = format!("{}-{}-{}", year, month, day);
    let time_str = format!("{:02}:{:02}", now.hour(), now.minute());

    // Filsti
    let mut file_path = PathBuf::from(&vault_dir);
    file_path.push("10-Journal");
    file_path.push(&year);
    file_path.push(&month);
    fs::create_dir_all(&file_path).expect("Kunne ikke opprette katalogstruktur");
    file_path.push(format!("{}.md", date_str));
    // Les innhold
    let content = read_to_string(&file_path).expect("Kunne ikke lese fil");
    let mut lines: Vec<&str> = content.lines().collect();

    // Finn første forekomst av "# Logg"
    if let Some(index) = lines.iter().position(|line| line.trim() == "## 🕗") {
        // Hvis linjen etter ikke finnes, eller ikke er tom, sett inn en tom linje
        if lines.get(index + 1).map_or(true, |l| !l.trim().is_empty()) {
            lines.insert(index + 1, "");
            // Skriv hele filen på nytt
            let new_content = lines.join("\n") + "\n"; // sørg for ny linje til slutt
            write(&file_path, new_content).expect("Kunne ikke skrive tilbake til fil");
        }
    }

    // Skriv ny logglinje til slutten av filen
    let file = OpenOptions::new()
        .append(true)
        .open(&file_path)
        .expect("Kunne ikke åpne fil for skriving");
    let mut writer = BufWriter::new(file);
    writeln!(writer, "* {} {}", time_str, sentence).expect("Kunne ikke skrive logglinje");

    println!("Log accepted");
}

