use std::{env, sync::Arc, time::Duration};

use evil_lumios::{Event, State};
use teloxide::{prelude::Requester, Bot};

pub async fn event_loop(bot: Bot, state: State) -> anyhow::Result<()> {
    let mut receiver = state.sender.subscribe();
    let arc_bot = Arc::new(bot);
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
                        .unwrap_or("60".to_string())
                        .parse::<u64>()
                        .unwrap();
                    tokio::time::sleep(Duration::from_secs(interval)).await;
                    bot_clone
                        .delete_message(chat_id, message_id)
                        .await
                        .expect("Failed to delete message");
                });
            }
        }
    }

    Ok(())
}
