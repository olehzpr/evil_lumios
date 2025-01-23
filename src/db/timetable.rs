use diesel::{
    ExpressionMethods, JoinOnDsl, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl,
};

use crate::{
    bot::timetable::{Day, Week},
    schema,
};

use super::models::TimetableEntry;

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
    day.next();

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
        .filter(schema::timetable_entries::class_time.ge(current_time))
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
        .filter(schema::timetable_entries::class_time.ge(current_time))
        .select(schema::timetable_entries::all_columns)
        .order(schema::timetable_entries::class_time.asc())
        .offset(1)
        .first::<TimetableEntry>(conn)
        .optional()?;

    Ok(entry)
}
