use chrono::FixedOffset;

pub fn get_current_time() -> chrono::DateTime<FixedOffset> {
    let timezone_offset = chrono::FixedOffset::east_opt(2 * 3600).unwrap();
    let now = chrono::Utc::now().with_timezone(&timezone_offset);

    now
}
