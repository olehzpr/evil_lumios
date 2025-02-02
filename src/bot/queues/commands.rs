use crate::bot::ui;
use crate::db::models::NewQueue;
use crate::db::queue::create_queue;
use crate::db::StateWithConnection;
use crate::param;
use crate::{bot::handler::HandlerResult, delete_message, State};
use teloxide::{
    prelude::Requester,
    types::{ChatId, Message},
    Bot,
};

use super::QueueMessages;

pub async fn queue(bot: Bot, msg: Message, state: State) -> HandlerResult {
    let conn = &mut state.conn().await;
    let name = param!(bot, msg, state, String, "Вкажіть назву черги");

    let loading_msg = loading_message(&bot, msg.chat.id).await?;

    let new_queue = create_queue(
        conn,
        NewQueue {
            title: &name,
            chat_id: &msg.chat.id.to_string(),
            message_id: &loading_msg.id.to_string(),
            is_mixed: None,
            is_priority: false,
        },
    )?;
    bot.edit_regular_queue(
        loading_msg.chat.id,
        loading_msg.id,
        new_queue.id,
        &ui::queue::start_message(name, ui::queue::QueueType::Regular),
    )
    .await;
    Ok(())
}

pub async fn mixed(bot: Bot, msg: Message, state: State) -> HandlerResult {
    let conn = &mut state.conn().await;
    let name = param!(bot, msg, state, String, "Вкажіть назву черги");

    let loading_msg = loading_message(&bot, msg.chat.id).await?;

    let new_queue = create_queue(
        conn,
        NewQueue {
            title: &name,
            chat_id: &msg.chat.id.to_string(),
            message_id: &loading_msg.id.to_string(),
            is_mixed: Some(false),
            is_priority: false,
        },
    )?;

    bot.edit_mixed_queue(
        loading_msg.chat.id,
        loading_msg.id,
        new_queue.id,
        &ui::queue::start_message(name, ui::queue::QueueType::Mixed),
    )
    .await;
    Ok(())
}

pub async fn priority_queue(bot: Bot, msg: Message, state: State) -> HandlerResult {
    let conn = &mut state.conn().await;
    let name = param!(bot, msg, state, String, "Вкажіть назву черги");

    let loading_msg = loading_message(&bot, msg.chat.id).await?;

    let new_queue = create_queue(
        conn,
        NewQueue {
            title: &name,
            chat_id: &msg.chat.id.to_string(),
            message_id: &loading_msg.id.to_string(),
            is_mixed: None,
            is_priority: true,
        },
    )?;

    bot.edit_priority_queue(
        loading_msg.chat.id,
        loading_msg.id,
        new_queue.id,
        &ui::queue::start_message(name, ui::queue::QueueType::Priority),
    )
    .await;
    Ok(())
}

async fn loading_message(bot: &Bot, chat_id: ChatId) -> anyhow::Result<Message> {
    let msg = bot
        .send_message(chat_id, "Створення нової черги...")
        .await?;

    Ok(msg)
}
