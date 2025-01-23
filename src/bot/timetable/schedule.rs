use evil_lumios::State;
use teloxide::types::ChatId;

use crate::db::{models::TimetableEntry, timetable::get_full_timetable, StateWithConnection};

use super::{Day, Week};

pub async fn schedule_timetable(chat_id: ChatId, state: &State) -> anyhow::Result<()> {
    let conn = &mut state.conn().await;
    let entries = get_full_timetable(conn, &chat_id.to_string()).await?;

    for entry in entries {
        tokio::spawn(async move {
            notify(entry).await;
        });
    }
    Ok(())
}

async fn notify(entry: TimetableEntry) {
    let current_week = Week::current();
    let current_day = Day::current();
    let total_current_day = current_week * (current_day as u8 + 1);
    let total_target_day = (entry.week * (entry.day + 1)) as u8;
    let now = chrono::Utc::now();
    if total_current_day > total_target_day {
        let day_diff = 14 - total_current_day + total_target_day;
    } else {
        let day_diff = total_target_day - total_current_day;
    }
}
