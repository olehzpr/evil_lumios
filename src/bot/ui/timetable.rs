use std::collections::{BTreeMap, HashMap};

use crate::{
    bot::timetable::{Day, Week},
    models::timetable::TimetableEntryModel,
};

use super::utils::adapt_for_markdown;

pub fn day_view(entries: Vec<TimetableEntryModel>) -> String {
    let mut grouped_entries: BTreeMap<String, HashMap<String, Vec<TimetableEntryModel>>> =
        BTreeMap::new();

    for entry in entries {
        let time_key = entry.class_time.format("%H:%M").to_string();
        let type_key = class_type_label(&entry.class_type).to_string();

        grouped_entries
            .entry(time_key)
            .or_insert_with(HashMap::new)
            .entry(type_key)
            .or_insert_with(Vec::new)
            .push(entry);
    }

    let mut response = String::new();

    response.push_str("📅 *\\>\\> РОЗКЛАД \\<\\<* 📅\n\n");

    for (time, class_types) in grouped_entries {
        let type_keys: Vec<&String> = class_types.keys().collect();

        if type_keys.len() == 1 {
            response.push_str(&format!(
                "*{} {} {}*\n",
                get_time_emoji(&time),
                time,
                reverse_type_label(type_keys[0])
            ));

            for entry in class_types[type_keys[0]].iter() {
                response.push_str(&format!("┃ {}\n", entry_row_no_time(entry)));
            }
        } else {
            response.push_str(&format!("*{} {}*\n", get_time_emoji(&time), time));

            for (class_type, group) in class_types {
                response.push_str(&format!("{}\n", class_type));

                for entry in group {
                    response.push_str(&format!("┃ {}\n", entry_row_no_time(&entry)));
                }
            }
        }

        response.push('\n');
    }

    if response.is_empty() {
        response = random_response();
    }
    return response;
}

pub fn week_view(entries: Vec<TimetableEntryModel>) -> String {
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

pub fn edit_view(entries: Vec<TimetableEntryModel>) -> String {
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

pub fn entry_view(entry: Option<TimetableEntryModel>) -> String {
    if entry.is_none() {
        return adapt_for_markdown(format!("{}\n", random_response()));
    }
    let entry = entry.unwrap();
    let identifier = class_type_identifier(&entry.class_type);
    let time = entry.class_time.format("%H:%M").to_string();
    adapt_for_markdown(format!(
        "🔔 *> НАГАДУВАННЯ* < 🔔\n\n{} {}\nПочаток: {} {}\n\nПосилання на конференцію ⬇️",
        identifier,
        entry.class_name,
        time,
        get_time_emoji(&time)
    ))
}

pub fn update_link_view(entry: &TimetableEntryModel) -> String {
    adapt_for_markdown(format!(
        "Надішліть посилання для пари *{} {} {}*\n",
        class_type_identifier(&entry.class_type),
        entry.class_name,
        class_type_identifier(&entry.class_type),
    ))
}

fn entry_row(entry: &TimetableEntryModel, edit: bool) -> String {
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
            "[✏️](https://t.me/evil_lumios_bot?start=edit-timetable_{})",
            entry.id
        );
    }
    let formatted_time = entry.class_time.format("%H:%M").to_string();
    adapt_for_markdown(format!(
        "{} {} - {} {}\n",
        identifier, formatted_time, link, edit_link
    ))
}

fn entry_row_no_time(entry: &TimetableEntryModel) -> String {
    let short_name = entry
        .class_name
        .split(|c| c == '.' || c == ':')
        .next()
        .unwrap();
    let mut link = short_name.to_string();
    if let Some(entry_link) = &entry.link {
        link = format!("[{}]({})", short_name, entry_link);
    }
    adapt_for_markdown(format!("{}", link))
}

pub fn class_type_identifier(class_type: &str) -> &str {
    match class_type {
        "lec" => "🔵",
        "lab" => "🟢",
        "prac" => "🟠",
        _ => "🟣",
    }
}

pub fn class_type_label(class_type: &str) -> &'static str {
    match class_type {
        "lec" => "🔵 Лекція:",
        "lab" => "🟢 Лабораторна:",
        "prac" => "🟠 Практика:",
        _ => "🟣 Інше:",
    }
}

pub fn reverse_type_label(class_type: &str) -> &'static str {
    match class_type {
        "🔵 Лекція:" => " Лекція 🔵:",
        "🟢 Лабораторна:" => "Лабораторна 🟢:",
        "🟠 Практика:" => "Практика 🟠:",
        _ => "Інше 🟣:",
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

fn get_time_emoji(time: &String) -> String {
    let emoji = match time.as_str() {
        "08:30" => "🕣",
        "10:25" => "🕥",
        "12:20" => "🕧",
        "14:15" => "🕑",
        "16:10" => "🕓",
        "18:05" => "🕕",
        "20:00" => "🕖",
        _ => "🕘",
    };

    emoji.to_string()
}
