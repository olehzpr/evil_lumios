use std::{env, sync::Arc, time::Duration};

use teloxide::{prelude::Requester, Bot};

use crate::state::Event;

pub async fn delete_message(bot: Arc<Bot>, event: Event) -> anyhow::Result<()> {
    let Event::DeleteMessage {
        chat_id,
        message_id,
    } = event
    else {
        return Ok(());
    };
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
        bot.delete_message(chat_id, message_id).await.map_err(|e| {
            tracing::error!("Failed to delete message: {:?}", e);
        })
    });

    Ok(())
}
