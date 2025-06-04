use chrono::{Local, Duration, Datelike, Weekday, Locale};
use std::fs;
use std::path::PathBuf;
use crate::config::Config;

pub struct TemplateData {
    pub today_date: String,
    pub yesterday_date: String,
    pub tomorrow_date: String,
    pub weekday: String,
}

impl TemplateData {
    fn map_locale(locale_str: &str) -> Option<Locale> {
        match locale_str {
            "en_US" => Some(Locale::en_US),
            "nb_NO" => Some(Locale::nb_NO),  // Norwegian Bokmål
            "nn_NO" => Some(Locale::nn_NO),  // Norwegian Nynorsk
            "de_DE" => Some(Locale::de_DE),  // German
            "fr_FR" => Some(Locale::fr_FR),  // French
            "es_ES" => Some(Locale::es_ES),  // Spanish
            "it_IT" => Some(Locale::it_IT),  // Italian
            "ja_JP" => Some(Locale::ja_JP),  // Japanese
            "ko_KR" => Some(Locale::ko_KR),  // Korean
            "ru_RU" => Some(Locale::ru_RU),  // Russian
            "zh_CN" => Some(Locale::zh_CN),  // Chinese
            _ => Some(Locale::nb_NO),
        }
    }

    fn get_weekday_name(weekday: Weekday, locale: Locale) -> String {
        // Create a known date for this weekday (using 2024-01-01 as Monday)
        let base_monday = chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let days_to_add = match weekday {
            Weekday::Mon => 0,
            Weekday::Tue => 1,
            Weekday::Wed => 2,
            Weekday::Thu => 3,
            Weekday::Fri => 4,
            Weekday::Sat => 5,
            Weekday::Sun => 6,
        };
        let target_date = base_monday + Duration::days(days_to_add);
        target_date.format_localized("%A", locale).to_string().to_lowercase()
    }

    pub fn new(locale_str: Option<&str>) -> Self {
        let now = Local::now();
        let today = now.date_naive();
        let yesterday = today - Duration::days(1);
        let tomorrow = today + Duration::days(1);

        // Get weekday name based on locale
        let weekday = match locale_str {
            Some(loc) => {
                match Self::map_locale(loc) {
                    Some(locale) => Self::get_weekday_name(today.weekday(), locale),
                    None => Self::weekday_to_string(today.weekday())
                }
            },
            None => Self::weekday_to_string(today.weekday()),
        };

        Self {
            today_date: today.format("%Y-%m-%d").to_string(),
            yesterday_date: yesterday.format("%Y-%m-%d").to_string(),
            tomorrow_date: tomorrow.format("%Y-%m-%d").to_string(),
            weekday,
        }
    }

    // Fallback English weekday names
    fn weekday_to_string(weekday: Weekday) -> String {
        match weekday {
            Weekday::Mon => "monday",
            Weekday::Tue => "tuesday",
            Weekday::Wed => "wednesday",
            Weekday::Thu => "thursday",
            Weekday::Fri => "friday",
            Weekday::Sat => "saturday",
            Weekday::Sun => "sunday",
        }.to_string()
    }
}

pub fn process_template(template_path: &str, data: &TemplateData) -> String {
    // Expand ~ to home directory if present
    let expanded_path = if template_path.starts_with("~") {
        if let Ok(home) = std::env::var("HOME") {
            let path = template_path.strip_prefix("~").unwrap_or(template_path);
            PathBuf::from(home).join(path.strip_prefix("/").unwrap_or(path))
        } else {
            PathBuf::from(template_path)
        }
    } else {
        PathBuf::from(template_path)
    };

    let template = match fs::read_to_string(&expanded_path) {
        Ok(content) => content,
        Err(_) => String::from("## 🕗\n\n"),
    };

    template
        .replace("{today}", &data.today_date)
        .replace("{yesterday}", &data.yesterday_date)
        .replace("{tomorrow}", &data.tomorrow_date)
        .replace("{weekday}", &data.weekday)
}

pub fn get_template_content(config: &Config) -> String {
    let template_data = TemplateData::new(config.locale.as_deref());
    
    match &config.template_path {
        Some(path) => process_template(path, &template_data),
        None => String::from("## 🕗\n\n"),
    }
} 