use crate::{bot::handler::HandlerResult, db::timetable::get_entry_by_id, state::State};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use reqwest::Url;
use teloxide::{
    payloads::EditMessageReplyMarkupSetters,
    prelude::{Request, Requester},
    types::{ChatId, InlineKeyboardButton, InlineKeyboardMarkup, Message, MessageId},
    Bot,
};

use crate::{
    config::state::{BotDialogue, StateMachine},
    db::StateWithConnection,
    schema,
};

pub async fn receive_timetable_entry_link(
    bot: Bot,
    dialogue: BotDialogue,
    msg: Message,
    id: i32,
    state: State,
) -> HandlerResult {
    match msg.text() {
        Some(link) => {
            if !link.starts_with("http") {
                bot.send_message(
                    msg.chat.id,
                    "Неправильне посилання, надішліть посилання що починається з http:// або https://",
                )
                .await?;
                dialogue
                    .update(StateMachine::ReceiveEditTimetableEntry { id })
                    .await?;
                return Ok(());
            }
            let conn = &mut state.conn().await;
            diesel::update(schema::timetable_entries::table.find(id))
                .set(schema::timetable_entries::link.eq(link))
                .execute(conn)?;
            bot.send_message(msg.chat.id, "Посилання успішно змінено")
                .await?;
            dialogue.exit().await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Надішліть посилання повторно")
                .await?;
        }
    }
    Ok(())
}

pub async fn receive_timetable_entry_link_from_message(
    bot: Bot,
    dialogue: BotDialogue,
    msg: Message,
    (id, chat_id, message_id): (i32, ChatId, MessageId),
    state: State,
) -> HandlerResult {
    let conn = &mut state.conn().await;
    receive_timetable_entry_link(bot.clone(), dialogue, msg, id, state).await?;
    tracing::debug!("Editing message");
    let entry = get_entry_by_id(conn, id)?;
    if entry.is_none() {
        return Ok(());
    }
    let entry = entry.unwrap();
    bot.edit_message_reply_markup(chat_id, message_id)
        .reply_markup(InlineKeyboardMarkup::new(vec![vec![
            InlineKeyboardButton::url(
                "Туда нам нада 🌐",
                Url::parse(&entry.link.unwrap_or_default()).unwrap(),
            ),
        ]]))
        .send()
        .await?;
    Ok(())
}
