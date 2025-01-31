use chrono::Timelike;
use teloxide::types::ChatId;

use crate::{
    db::{
        chat::get_chats, models::TimetableEntry, timetable::get_full_timetable, StateWithConnection,
    },
    state::{CacheValue, Event, State},
};

use super::{Day, Week};

pub async fn timetable_notifications(state: State) {
    let chat_ids = get_chat_ids(&state).await.unwrap();
    for chat_id in chat_ids {
        let entries = match get_entries(&state, chat_id.to_string()).await {
            Ok(entries) => entries,
            Err(err) => {
                eprintln!("Failed to get entries: {}", err);
                return;
            }
        };
        let now = chrono::Utc::now();
        let current_week = Week::current() as i32;
        let current_day = Day::current() as i32;
        for entry in entries.iter() {
            if entry.week != current_week || entry.day != current_day {
                continue;
            }
            let class_time = entry.class_time;
            if (class_time.hour() as i32) == (now.hour() as i32)
                && (class_time.minute() as i32) - (now.minute() as i32) == 3
                && (class_time.second() as i32) - (now.second() as i32) < 60
            {
                _ = state.sender.send(Event::Notify {
                    chat_id,
                    entry_id: entry.id,
                });
            }
        }
    }
}

async fn get_chat_ids(state: &State) -> anyhow::Result<Vec<ChatId>> {
    let conn = &mut state.conn().await;
    let key_string = "chat_ids";
    let chat_ids: Vec<ChatId>;
    if let Some(cache_value) = state.cache.get(key_string) {
        chat_ids = match cache_value.value() {
            CacheValue::ChatIds(chat_ids) => chat_ids.clone(),
            _ => get_chats(conn).await?,
        };
    } else {
        chat_ids = get_chats(conn).await?;
        state.cache.insert(
            key_string.to_string(),
            CacheValue::ChatIds(chat_ids.clone()),
        );
    }

    Ok(chat_ids)
}

async fn get_entries(state: &State, chat_id: String) -> anyhow::Result<Vec<TimetableEntry>> {
    let conn = &mut state.conn().await;
    let entries: Vec<TimetableEntry>;
    let key_string = format!("schedule_{}", chat_id);
    let key = key_string.as_str();
    if let Some(cache_value) = state.cache.get(key) {
        entries = match cache_value.value() {
            CacheValue::TimetableEntries(stored_entries) => stored_entries.clone(),
            _ => get_full_timetable(conn, &chat_id).await?,
        };
    } else {
        entries = get_full_timetable(conn, &chat_id).await?;
        state.cache.insert(
            key.to_string(),
            CacheValue::TimetableEntries(entries.clone()),
        );
    }

    Ok(entries)
}
