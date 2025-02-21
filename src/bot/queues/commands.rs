use crate::bot::queues::QueueMessages;
use crate::bot::ui;
use crate::db::queue::create_queue;
use crate::delete_message;
use crate::entities::queues;
use crate::state::State;
use crate::{bot::handler::HandlerResult, param};
use sea_orm::entity::*;
use teloxide::{
    prelude::Requester,
    types::{ChatId, Message},
    Bot,
};

pub async fn queue(bot: Bot, msg: Message, state: State) -> HandlerResult {
    let name = param!(bot, msg, state, String, "Вкажіть назву черги");

    let loading_msg = loading_message(&bot, msg.chat.id).await?;

    let new_queue = create_queue(
        &state.db,
        queues::ActiveModel {
            title: Set(name.clone()),
            chat_id: Set(msg.chat.id.to_string()),
            message_id: Set(loading_msg.id.to_string()),
            is_mixed: Set(None),
            is_priority: Set(false),
            ..Default::default()
        },
    )
    .await?;

    bot.edit_regular_queue(
        loading_msg.chat.id,
        loading_msg.id,
        new_queue.id,
        &ui::queue::start_message(name, ui::queue::QueueType::Regular),
    )
    .await;

    delete_message!(state, msg);
    Ok(())
}

pub async fn mixed(bot: Bot, msg: Message, state: State) -> HandlerResult {
    let name = param!(bot, msg, state, String, "Вкажіть назву черги");

    let loading_msg = loading_message(&bot, msg.chat.id).await?;

    let new_queue = create_queue(
        &state.db,
        queues::ActiveModel {
            title: Set(name.clone()),
            chat_id: Set(msg.chat.id.to_string()),
            message_id: Set(loading_msg.id.to_string()),
            is_mixed: Set(Some(true)),
            is_priority: Set(false),
            ..Default::default()
        },
    )
    .await?;

    bot.edit_mixed_queue(
        loading_msg.chat.id,
        loading_msg.id,
        new_queue.id,
        &ui::queue::start_message(name, ui::queue::QueueType::Mixed),
    )
    .await;

    delete_message!(state, msg);
    Ok(())
}

pub async fn priority_queue(bot: Bot, msg: Message, state: State) -> HandlerResult {
    let name = param!(bot, msg, state, String, "Вкажіть назву черги");

    let loading_msg = loading_message(&bot, msg.chat.id).await?;

    let new_queue = create_queue(
        &state.db,
        queues::ActiveModel {
            title: Set(name.clone()),
            chat_id: Set(msg.chat.id.to_string()),
            message_id: Set(loading_msg.id.to_string()),
            is_mixed: Set(None),
            is_priority: Set(true),
            ..Default::default()
        },
    )
    .await?;

    bot.edit_priority_queue(
        loading_msg.chat.id,
        loading_msg.id,
        new_queue.id,
        &ui::queue::start_message(name, ui::queue::QueueType::Priority),
    )
    .await;

    delete_message!(state, msg);
    Ok(())
}

async fn loading_message(bot: &Bot, chat_id: ChatId) -> anyhow::Result<Message> {
    let msg = bot
        .send_message(chat_id, "Створення нової черги...")
        .await?;

    Ok(msg)
}
