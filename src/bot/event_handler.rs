use std::{env, sync::Arc, time::Duration};

use crate::{
    db::{timetable::get_entry_by_id, StateWithConnection},
    send_temp,
    state::{Event, State},
};
use teloxide::{prelude::Requester, Bot};

use super::{
    externsions::{ExtendedBot, Msg},
    ui,
};

pub async fn event_loop(bot: Bot, state: State) -> anyhow::Result<()> {
    let mut receiver = state.sender.subscribe();
    let arc_bot = Arc::new(bot);
    tracing::info!("Starting event loop");
    while let Ok(event) = receiver.recv().await {
        match event {
            Event::Exit => {
                break;
            }
            Event::DeleteMessage {
                chat_id,
                message_id,
            } => {
                let bot_clone = arc_bot.clone();
                tokio::spawn(async move {
                    let interval = env::var("MESSAGE_CLEANUP_INTERVAL")
                        .unwrap_or_else(|_| {
                            tracing::warn!("Environment variable MESSAGE_CLEANUP_INTERVAL is not set, using default value of 60 seconds");
                            "60".to_string()
                        })
                        .parse::<u64>()
                        .unwrap_or_else(|_| {
                            tracing::warn!("Environment variable MESSAGE_CLEANUP_INTERVAL is not a number, using default value of 60 seconds");
                            60
                        });
                    tokio::time::sleep(Duration::from_secs(interval)).await;
                    bot_clone
                        .delete_message(chat_id, message_id)
                        .await
                        .expect("Failed to delete message");
                });
            }
            Event::Notify { chat_id, entry_id } => {
                let bot = arc_bot.clone();
                let conn = &mut state.conn().await;
                let entry = get_entry_by_id(conn, entry_id)?;
                let res = ui::timetable::entry_view(entry);
                send_temp!(bot, state, chat_id, &res);
            }
        }
    }

    Ok(())
}
