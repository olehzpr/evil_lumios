use crate::bot::handler::HandlerResult;
use crate::delete_message;
use crate::state::Event;
use crate::{
    bot::ui,
    db::{stats::get_user_stats, StateWithConnection},
    State,
};
use reqwest::Url;
use teloxide::payloads::{EditMessageReplyMarkupSetters, SendMessageSetters, SendPhotoSetters};
use teloxide::prelude::Request;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, InputFile};
use teloxide::{prelude::Requester, types::Message, Bot};

pub async fn stats(bot: Bot, msg: Message, _state: State) -> HandlerResult {
    bot.send_message(msg.chat.id, "Stats command").await?;
    Ok(())
}

pub async fn casino(bot: Bot, msg: Message, state: State) -> HandlerResult {
    let (res, url) = ui::stats::casino_welcome();
    let bot_name = bot.get_me().await?.user.username.unwrap();
    let new_msg = bot
        .send_photo(msg.chat.id, InputFile::url(Url::parse(&url).unwrap()))
        .caption(res)
        .reply_markup(InlineKeyboardMarkup::new(vec![vec![
            InlineKeyboardButton::url(
                "Пройти до казино",
                Url::parse(&format!("https://t.me/{}?start=casino", bot_name)).unwrap(),
            ),
        ]]))
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .send()
        .await?;

    delete_message!(state, msg);
    delete_message!(state, new_msg);

    Ok(())
}

pub async fn me(bot: Bot, msg: Message, state: State) -> HandlerResult {
    let conn = &mut state.conn().await;
    let user_id = msg.from.as_ref().unwrap().id;
    let stats = get_user_stats(conn, msg.from.unwrap().id).await?;
    let res = ui::stats::short_stats(stats);
    if let Err(e) = state.sender.send(Event::DeleteMessage {
        chat_id: msg.chat.id,
        message_id: msg.id,
    }) {
        eprintln!("Failed to send delete message event: {:?}", e);
    }

    let sent_msg = bot
        .send_message(msg.chat.id, &res)
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .reply_markup(InlineKeyboardMarkup::new(vec![vec![
            InlineKeyboardButton::callback("Показати всю статистику", "loading"),
        ]]))
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

    delete_message!(state, msg);
    delete_message!(state, sent_msg);

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
