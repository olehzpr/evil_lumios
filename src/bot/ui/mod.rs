use crate::db::models::TimetableEntry;

pub fn class_type_identifier(class_type: &str) -> &str {
    match class_type {
        "lec" => "🔵",
        "lab" => "🟢",
        "prac" => "🟠",
        _ => "🟣",
    }
}

pub fn adapt_for_markdown(msg: String) -> String {
    msg.replace("_", "\\_")
        .replace("`", "\\`")
        .replace("~", "\\~")
        .replace(">", "\\>")
        .replace("#", "\\#")
        .replace("+", "\\+")
        .replace("-", "\\-")
        .replace("=", "\\=")
        .replace("|", "\\|")
        .replace("{", "\\{")
        .replace("}", "\\}")
        .replace(".", "\\.")
}

pub fn small_timetable_entry_view(entry: &TimetableEntry, edit: bool) -> String {
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

pub fn extedned_timetable_entry_view(entry: &TimetableEntry) -> String {
    let identifier = class_type_identifier(&entry.class_type);
    adapt_for_markdown(format!(
        "{} {}: {} {}\n",
        identifier, entry.class_name, entry.class_type, entry.class_time
    ))
}

pub fn change_timetable_entry_request_view(entry: &TimetableEntry) -> String {
    adapt_for_markdown(format!(
        "Надішліть посилання для пари *{} {} {}*\n",
        class_type_identifier(&entry.class_type),
        entry.class_name,
        class_type_identifier(&entry.class_type),
    ))
}
