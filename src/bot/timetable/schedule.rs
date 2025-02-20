use chrono::Timelike;
use teloxide::types::ChatId;

use crate::{
    bot::utils::time::get_current_time,
    db::{self, timetable::get_full_timetable},
    entities::timetable_entries::Model as TimetableEntry,
    redis::RedisCache,
    state::{Event, State},
};

use super::{Day, Week};

pub async fn timetable_notifications(state: State) {
    let chat_ids = get_chat_ids(&state).await.unwrap();
    for chat_id in chat_ids {
        let entries = match get_entries(&state, chat_id).await {
            Ok(entries) => entries,
            Err(err) => {
                eprintln!("Failed to get entries: {}", err);
                return;
            }
        };
        let now = get_current_time();
        let current_week = Week::current() as i32;
        let current_day = Day::current() as i32;
        for entry in entries.iter() {
            if entry.week != current_week || entry.day != current_day {
                continue;
            }
            let class_time = entry.class_time;
            if (class_time.hour() as i32) == (now.hour() as i32)
                && (class_time.minute() as i32) - (now.minute() as i32) == 3
            {
                _ = state.sender.send(Event::NotifyTimetable {
                    chat_id,
                    entry_id: entry.id,
                });
            }
        }
    }
}

async fn get_chat_ids(state: &State) -> anyhow::Result<Vec<ChatId>> {
    if let Ok(chat_ids) = state.redis.get_all_chat_ids() {
        return Ok(chat_ids);
    }
    let chat_ids = db::chat::get_chat_ids(&state.db).await?;
    state.redis.store_chat_ids(chat_ids.clone())?;
    Ok(chat_ids)
}

async fn get_entries(state: &State, chat_id: ChatId) -> anyhow::Result<Vec<TimetableEntry>> {
    if let Ok(entries) = state.redis.get_timetable_entries(chat_id) {
        return Ok(entries);
    }
    let entries = get_full_timetable(&state.db, &chat_id.to_string()).await?;
    state
        .redis
        .store_timetable_entries(chat_id, entries.clone())?;
    Ok(entries)
}
