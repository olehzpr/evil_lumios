use diesel::{ExpressionMethods, JoinOnDsl, QueryDsl, RunQueryDsl};
use evil_lumios::State;
use teloxide::types::ChatId;

use crate::{
    db::{connection, models::TimetableEntry},
    schema,
};

use super::utils::{get_current_day, get_current_week};

pub async fn schedule_timetable(chat_id: ChatId, state: &State) -> anyhow::Result<()> {
    let conn = &mut connection(state).await;
    let entries = schema::timetable_entries::table
        .inner_join(
            schema::timetables::table
                .on(schema::timetable_entries::timetable_id.eq(schema::timetables::id)),
        )
        .filter(schema::timetables::chat_id.eq(chat_id.to_string()))
        .select(schema::timetable_entries::all_columns)
        .order((
            schema::timetable_entries::week.asc(),
            schema::timetable_entries::day.asc(),
            schema::timetable_entries::class_time.asc(),
        ))
        .load::<TimetableEntry>(conn)?;

    for entry in entries {
        tokio::spawn(async move {
            notify(entry).await;
        });
    }
    Ok(())
}

async fn notify(entry: TimetableEntry) {
    let current_week = get_current_week();
    let current_day = get_current_day();
    let total_current_day = current_week * (current_day + 1);
    let total_target_day = (entry.week * (entry.day + 1)) as u8;
    let now = chrono::Utc::now();
    if total_current_day > total_target_day {
        let day_diff = 14 - total_current_day + total_target_day;
    } else {
        let day_diff = total_target_day - total_current_day;
    }
}
