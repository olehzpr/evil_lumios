use std::sync::Arc;

use crate::state::{Event, State};
use teloxide::Bot;

pub mod cleanup;
pub mod gamble;
pub mod notification;

pub async fn event_loop(bot: Bot, state: State) -> anyhow::Result<()> {
    let bot = Arc::new(bot);
    let mut receiver = state.sender.subscribe();
    tracing::info!("Starting event loop");
    while let Ok(event) = receiver.recv().await {
        let bot = bot.clone();
        let state = state.clone();
        match event {
            Event::Exit => break,
            Event::DeleteMessage { .. } => {
                cleanup::delete_message(bot, event).await?;
            }
            Event::NotifyTimetable { .. } => {
                notification::notify(bot, state, event).await?;
            }
            Event::GambleResult { .. } => {
                gamble::show_gamble_result(bot, state, event).await?;
            }
        }
    }
    Ok(())
}
