use std::{sync::Arc, time::Duration};

use teloxide::{
    payloads::EditMessageCaptionSetters,
    prelude::{Request, Requester},
    types::MessageId,
    Bot,
};

use crate::{
    bot::ui,
    db,
    state::{Event, State},
};

pub async fn show_gamble_result(bot: Arc<Bot>, state: State, event: Event) -> anyhow::Result<()> {
    let Event::GambleResult { chat_id, gamble_id } = event else {
        return Ok(());
    };

    let gamble = db::gamble::get_gamble_by_id(&state.db, gamble_id).await?;
    if let None = gamble {
        return Ok(());
    }
    let gamble = gamble.unwrap();

    let content = if gamble.is_win {
        ui::stats::generate_win_message(gamble.bet, gamble.bet + gamble.change)
    } else {
        ui::stats::generate_lose_message(gamble.bet, gamble.bet + gamble.change)
    };
    let message_id = MessageId(gamble.message_id);
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(8)).await;
        bot.edit_message_caption(chat_id, message_id)
            .caption(&content)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to edit message caption: {:?}", e);
            })
    });

    Ok(())
}
