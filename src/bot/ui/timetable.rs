use crate::db::models::TimetableEntry;

use super::utils::adapt_for_markdown;

pub fn day_view(entries: Vec<TimetableEntry>) -> String {
    let mut response = String::new();
    for entry in entries {
        response.push_str(&small_timetable_entry_view(&entry, false));
    }
    return response;
}
pub fn week_view(entries: Vec<TimetableEntry>) -> String {
    let mut response = String::new();
    let mut day: u8 = 0;
    for entry in entries {
        if entry.day == day as i32 {
            response.push_str(&format!("\n*{}*\n", day.to_string()));
            day += 1;
        }
        response.push_str(&small_timetable_entry_view(&entry, false));
    }
    if response.is_empty() {
        response = "Схоже що на цей тиждень немає жодних пар. Спробуйте імпортувати розклад за допомогою команди /import".to_string();
    }
    return response;
}

pub fn edit_view(entries: Vec<TimetableEntry>) -> String {
    let mut response = String::new();
    let mut day: u8 = 0;
    let mut week: u8 = 1;
    for entry in entries {
        if entry.week == week as i32 {
            response.push_str(&format!("\n*📅 Week {}*\n", week));
            week += 1;
            day = 0;
        }
        if entry.day == day as i32 {
            response.push_str(&format!("\n*{}*\n", day.to_string()));
            day += 1;
        }
        response.push_str(&small_timetable_entry_view(&entry, true));
    }
    return response;
}

pub fn entry_view(entry: &TimetableEntry) -> String {
    let identifier = class_type_identifier(&entry.class_type);
    adapt_for_markdown(format!(
        "{} {}: {} {}\n",
        identifier, entry.class_name, entry.class_type, entry.class_time
    ))
}

fn small_timetable_entry_view(entry: &TimetableEntry, edit: bool) -> String {
    let identifier = class_type_identifier(&entry.class_type);
    let short_name = entry
        .class_name
        .split(|c| c == '.' || c == ':')
        .next()
        .unwrap();
    let mut link = short_name.to_string();
    let mut edit_link = "".to_string();
    if let Some(entry_link) = &entry.link {
        link = format!("[{}]({})", short_name, entry_link);
    }
    if edit {
        edit_link = format!(
            "[✏️](https://t.me/evil_lumios_bot?start=edit_timetable_{})",
            entry.id
        );
    }
    let formatted_time = entry.class_time.format("%H:%M").to_string();
    adapt_for_markdown(format!(
        "{} {} - {} {}\n",
        identifier, formatted_time, link, edit_link
    ))
}

pub fn update_link_view(entry: &TimetableEntry) -> String {
    adapt_for_markdown(format!(
        "Надішліть посилання для пари *{} {} {}*\n",
        class_type_identifier(&entry.class_type),
        entry.class_name,
        class_type_identifier(&entry.class_type),
    ))
}

pub fn class_type_identifier(class_type: &str) -> &str {
    match class_type {
        "lec" => "🔵",
        "lab" => "🟢",
        "prac" => "🟠",
        _ => "🟣",
    }
}
