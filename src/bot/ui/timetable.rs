use crate::{
    bot::timetable::{Day, Week},
    db::models::TimetableEntry,
};

use super::utils::adapt_for_markdown;

pub fn day_view(entries: Vec<TimetableEntry>) -> String {
    let mut response = String::new();
    for entry in entries {
        response.push_str(&entry_row(&entry, false));
    }
    if response.is_empty() {
        response = random_response();
    }
    return response;
}
pub fn week_view(entries: Vec<TimetableEntry>) -> String {
    let mut response = String::new();
    let mut day: Day = Day::Mon;
    for entry in entries {
        if entry.day == day as i32 {
            response.push_str(&format!("\n*{}*\n", day));
            day = day.next();
        }
        response.push_str(&entry_row(&entry, false));
    }
    if response.is_empty() {
        response = adapt_for_markdown("Схоже що на цей тиждень немає жодних пар. Спробуйте імпортувати розклад за допомогою команди /import\n".to_string());
    }
    return response;
}

pub fn edit_view(entries: Vec<TimetableEntry>) -> String {
    let mut response = String::new();
    let mut day: Day = Day::Mon;
    let mut week: Week = Week::First;
    for entry in entries {
        if entry.week == u8::from(week) as i32 {
            response.push_str(&format!("\n*📅 {}*\n", week));
            week = week.next();
            day = Day::Mon;
        }
        if entry.day == u8::from(day) as i32 {
            response.push_str(&format!("\n*{}*\n", day.to_string()));
            day = day.next();
        }
        response.push_str(&entry_row(&entry, true));
    }
    return response;
}

pub fn entry_view(entry: Option<TimetableEntry>) -> String {
    if entry.is_none() {
        return adapt_for_markdown(format!("{}\n", random_response()));
    }
    let entry = entry.unwrap();
    let identifier = class_type_identifier(&entry.class_type);
    adapt_for_markdown(format!(
        "{} {}: {} {}\n",
        identifier, entry.class_name, entry.class_type, entry.class_time
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

fn entry_row(entry: &TimetableEntry, edit: bool) -> String {
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

pub fn class_type_identifier(class_type: &str) -> &str {
    match class_type {
        "lec" => "🔵",
        "lab" => "🟢",
        "prac" => "🟠",
        _ => "🟣",
    }
}

pub fn random_response() -> String {
    let responses = vec![
        "Наразі заняття відсутні. Може, краще прогуляйся замість сидіння тут 📡",
        "Все ще немає заняття. Спробуй хоча б почитати щось корисне 📡",
        "Тут нічого немає. Може, вже час знайти собі хобі? 📡",
        "Займатися нема чим? Піди поприбирай або зроби щось, що не соромно розповісти друзям 📡",
        "Жодних занять. Напевно, це знак, що пора зробити щось із життя 📡",
        "Тут порожньо. Можливо, варто врешті прокачати свій мозок, а не скролити 📡",
        "Нічого цікавого. Як щодо того, щоб припинити бути ледарем і рухатись уперед 📡",
        "Занять немає, але ти ж не безнадійний, знайди собі справу 📡",
    ];
    return responses[rand::random::<usize>() % responses.len()].to_string();
}
