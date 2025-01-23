use chrono::Datelike;

pub fn get_current_week() -> u8 {
    let now = chrono::Utc::now();
    let start = chrono::NaiveDate::from_isoywd_opt(now.year(), 1, chrono::Weekday::Mon);
    let week_number = now.date_naive().iso_week().week() - start.unwrap().iso_week().week() + 1;
    (week_number % 2 + 1) as u8
}

pub fn get_current_day() -> u8 {
    let now = chrono::Utc::now();
    let day = now.date_naive().weekday();
    match day {
        chrono::Weekday::Mon => 0,
        chrono::Weekday::Tue => 1,
        chrono::Weekday::Wed => 2,
        chrono::Weekday::Thu => 3,
        chrono::Weekday::Fri => 4,
        chrono::Weekday::Sat => 5,
        chrono::Weekday::Sun => 6,
    }
}

pub fn get_day_name(day: u8) -> &'static str {
    match day {
        0 => "Понеділок",
        1 => "Вівторок",
        2 => "Середа",
        3 => "Четвер",
        4 => "П'ятниця",
        5 => "Субота",
        6 => "Неділя",
        _ => "Невідомий день",
    }
}
