use std::sync::Arc;

use crate::{
    schema,
    state::{CacheValue, Event, State},
};
use chrono::Timelike;
use diesel::{QueryDsl, RunQueryDsl};
use log::info;
use teloxide::types::ChatId;
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::db::{models::TimetableEntry, timetable::get_full_timetable, StateWithConnection};

use super::{Day, Week};

pub async fn schedule_all_timetables(state: State) -> anyhow::Result<()> {
    let conn = &mut state.conn().await;

    let distinct_chat_ids = schema::timetables::table
        .select(schema::timetables::chat_id)
        .distinct()
        .load::<String>(conn)?;

    for chat_id in distinct_chat_ids {
        schedule_timetable(ChatId(chat_id.parse().unwrap()), state.clone()).await?;
    }
    Ok(())
}

pub async fn schedule_timetable(chat_id: ChatId, state: State) -> anyhow::Result<()> {
    let scheduler = JobScheduler::new().await?;

    let state_clone = Arc::new(state);

    info!("Scheduling timetable for chat_id: {}", chat_id);

    let job = Job::new_async("1/10 * * * * *", move |_uuid, _lock| {
        let state = Arc::clone(&state_clone);
        Box::pin(async move {
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
        })
    })?;

    scheduler.add(job).await?;

    scheduler.start().await?;
    Ok(())
}

async fn get_entries(state: &Arc<State>, chat_id: String) -> anyhow::Result<Vec<TimetableEntry>> {
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
