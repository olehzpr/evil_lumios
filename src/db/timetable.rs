use diesel::{
    ExpressionMethods, JoinOnDsl, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl,
};
use serde_json::Value;

use crate::{
    bot::timetable::{Day, Week},
    schema,
};

use super::models::{NewTimetable, NewTimetableEntry, Timetable, TimetableEntry};

pub async fn import_timetable(
    conn: &mut PgConnection,
    chat_id: &str,
    timetable: Value,
) -> anyhow::Result<()> {
    let existing_timetable = schema::timetables::table
        .filter(schema::timetables::chat_id.eq(chat_id))
        .first::<Timetable>(conn)
        .optional()?;
    if let Some(existing_timetable) = existing_timetable {
        diesel::delete(schema::timetable_entries::table)
            .filter(schema::timetable_entries::timetable_id.eq(existing_timetable.id))
            .execute(conn)?;
        diesel::delete(schema::timetables::table)
            .filter(schema::timetables::chat_id.eq(chat_id))
            .execute(conn)?;
    }

    let created_timetable = diesel::insert_into(schema::timetables::table)
        .values(NewTimetable { chat_id: chat_id })
        .get_result::<Timetable>(conn)?;
    let mut entries: Vec<NewTimetableEntry> = vec![];
    for (week, schedule_key) in [
        (Week::First, "scheduleFirstWeek"),
        (Week::Second, "scheduleSecondWeek"),
    ] {
        if let Some(days) = timetable["data"][schedule_key].as_array() {
            for (index, day) in days.iter().enumerate() {
                if let Some(pairs) = day["pairs"].as_array() {
                    for entry in pairs {
                        entries.push(NewTimetableEntry {
                            timetable_id: created_timetable.id,
                            week: week as i32,
                            day: index as i32,
                            class_name: entry["name"].as_str().unwrap(),
                            class_type: entry["tag"].as_str().unwrap(),
                            class_time: chrono::NaiveTime::parse_from_str(
                                entry["time"].as_str().unwrap(),
                                "%H:%M",
                            )
                            .unwrap(),
                            link: None,
                        });
                    }
                }
            }
        }
    }
    entries.iter().for_each(|entry| {
        diesel::insert_into(schema::timetable_entries::table)
            .values(entry)
            .execute(conn)
            .unwrap();
    });

    Ok(())
}

pub async fn get_today_timetable(
    conn: &mut PgConnection,
    chat_id: &str,
) -> anyhow::Result<Vec<TimetableEntry>> {
    let week = Week::current();
    let day = Day::current();
    let entries = schema::timetable_entries::table
        .inner_join(
            schema::timetables::table
                .on(schema::timetable_entries::timetable_id.eq(schema::timetables::id)),
        )
        .filter(schema::timetables::chat_id.eq(chat_id))
        .filter(schema::timetable_entries::week.eq(week as i32))
        .filter(schema::timetable_entries::day.eq(day as i32))
        .select(schema::timetable_entries::all_columns)
        .load::<TimetableEntry>(conn)?;

    return Ok(entries);
}

pub async fn get_tomorrow_timetable(
    conn: &mut PgConnection,
    chat_id: &str,
) -> anyhow::Result<Vec<TimetableEntry>> {
    let mut week = Week::current();
    let day = Day::current();
    if day == Day::Sat {
        week = week.next();
    }
    let day = day.next();

    let entries = schema::timetable_entries::table
        .inner_join(
            schema::timetables::table
                .on(schema::timetable_entries::timetable_id.eq(schema::timetables::id)),
        )
        .filter(schema::timetables::chat_id.eq(chat_id))
        .filter(schema::timetable_entries::week.eq(week as i32))
        .filter(schema::timetable_entries::day.eq(day as i32))
        .select(schema::timetable_entries::all_columns)
        .load::<TimetableEntry>(conn)?;

    Ok(entries)
}

pub async fn get_week_timetable(
    conn: &mut PgConnection,
    chat_id: &str,
) -> anyhow::Result<Vec<TimetableEntry>> {
    let week = Week::current();
    let entries = schema::timetable_entries::table
        .inner_join(
            schema::timetables::table
                .on(schema::timetable_entries::timetable_id.eq(schema::timetables::id)),
        )
        .filter(schema::timetables::chat_id.eq(chat_id))
        .filter(schema::timetable_entries::week.eq(week as i32))
        .select(schema::timetable_entries::all_columns)
        .order((
            schema::timetable_entries::day.asc(),
            schema::timetable_entries::class_time.asc(),
        ))
        .load::<TimetableEntry>(conn)?;

    return Ok(entries);
}

pub async fn get_full_timetable(
    conn: &mut PgConnection,
    chat_id: &str,
) -> anyhow::Result<Vec<TimetableEntry>> {
    let entries = schema::timetable_entries::table
        .inner_join(
            schema::timetables::table
                .on(schema::timetable_entries::timetable_id.eq(schema::timetables::id)),
        )
        .filter(schema::timetables::chat_id.eq(chat_id))
        .select(schema::timetable_entries::all_columns)
        .order((
            schema::timetable_entries::week.asc(),
            schema::timetable_entries::day.asc(),
            schema::timetable_entries::class_time.asc(),
        ))
        .load::<TimetableEntry>(conn)?;

    return Ok(entries);
}

pub async fn get_current_entry(
    conn: &mut PgConnection,
    chat_id: &str,
) -> anyhow::Result<Option<TimetableEntry>> {
    let week = Week::current();
    let day = Day::current();
    let current_time = chrono::Utc::now().time();
    let entry = schema::timetable_entries::table
        .inner_join(
            schema::timetables::table
                .on(schema::timetable_entries::timetable_id.eq(schema::timetables::id)),
        )
        .filter(schema::timetables::chat_id.eq(chat_id))
        .filter(schema::timetable_entries::week.eq(week as i32))
        .filter(schema::timetable_entries::day.eq(day as i32))
        .filter(
            schema::timetable_entries::class_time.ge(current_time + chrono::Duration::minutes(5)),
        )
        .select(schema::timetable_entries::all_columns)
        .order(schema::timetable_entries::class_time.asc())
        .first::<TimetableEntry>(conn)
        .optional()?;

    Ok(entry)
}

pub async fn get_next_entry(
    conn: &mut PgConnection,
    chat_id: &str,
) -> anyhow::Result<Option<TimetableEntry>> {
    let week = Week::current();
    let day = Day::current();
    let current_time = chrono::Utc::now().time();
    let entry = schema::timetable_entries::table
        .inner_join(
            schema::timetables::table
                .on(schema::timetable_entries::timetable_id.eq(schema::timetables::id)),
        )
        .filter(schema::timetables::chat_id.eq(chat_id))
        .filter(schema::timetable_entries::week.eq(week as i32))
        .filter(schema::timetable_entries::day.eq(day as i32))
        .filter(
            schema::timetable_entries::class_time.ge(current_time + chrono::Duration::minutes(5)),
        )
        .select(schema::timetable_entries::all_columns)
        .order(schema::timetable_entries::class_time.asc())
        .offset(1)
        .first::<TimetableEntry>(conn)
        .optional()?;

    Ok(entry)
}

pub fn get_entry_by_id(
    conn: &mut PgConnection,
    entry_id: i32,
) -> anyhow::Result<Option<TimetableEntry>> {
    let entry = schema::timetable_entries::table
        .filter(schema::timetable_entries::id.eq(entry_id))
        .first::<TimetableEntry>(conn)
        .optional()?;

    Ok(entry)
}
