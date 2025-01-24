use crate::bot::externsions::{ExtendedBot, Msg};
use crate::bot::handler::HandlerResult;
use crate::state::Event;
use crate::{
    bot::ui,
    db::{stats::get_short_me, StateWithConnection},
    State,
};
use teloxide::payloads::EditMessageReplyMarkupSetters;
use teloxide::prelude::Request;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use teloxide::{prelude::Requester, types::Message, Bot};

pub async fn stats(bot: Bot, msg: Message, _state: State) -> HandlerResult {
    bot.send_message(msg.chat.id, "Stats command").await?;
    Ok(())
}

pub async fn me(bot: Bot, msg: Message, state: State) -> HandlerResult {
    let conn = &mut state.conn().await;
    let user_id = msg.from.as_ref().unwrap().id;
    let stats = get_short_me(conn, msg.from.unwrap().id).await?;
    let res = ui::stats::short_stats(stats);
    if let Err(e) = state.sender.send(Event::DeleteMessage {
        chat_id: msg.chat.id,
        message_id: msg.id,
    }) {
        eprintln!("Failed to send delete message event: {:?}", e);
    }

    let sent_msg = bot
        .send_with_keyboard(
            Msg::Temp(msg.chat.id, &res, state.sender.clone()),
            InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(
                "Показати всю статистику",
                "loading",
            )]]),
        )
        .await?;
    bot.edit_message_reply_markup(sent_msg.chat.id, sent_msg.id)
        .reply_markup(InlineKeyboardMarkup::new(vec![vec![
            InlineKeyboardButton::callback(
                "Показати всю статистику",
                format!("show-full-stats_{}_{}", user_id, sent_msg.id),
            ),
        ]]))
        .send()
        .await?;

    Ok(())
}

pub async fn wheel(bot: Bot, msg: Message, _state: State) -> HandlerResult {
    bot.send_message(msg.chat.id, "Wheel command").await?;
    Ok(())
}

pub async fn gamble(bot: Bot, msg: Message, _state: State) -> HandlerResult {
    bot.send_message(msg.chat.id, "Gamble command").await?;
    Ok(())
}

pub async fn gamble_all(bot: Bot, msg: Message, _state: State) -> HandlerResult {
    bot.send_message(msg.chat.id, "GambleAll command").await?;
    Ok(())
}
