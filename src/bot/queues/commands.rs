use crate::{bot::handler::HandlerResult, State};
use teloxide::{
    prelude::Requester,
    types::{ChatId, Message},
    Bot,
};

pub async fn queue(bot: Bot, msg: Message, _state: State) -> HandlerResult {
    let loading_msg = loading_message(&bot, msg.chat.id).await?;

    bot.edit_message_text(msg.chat.id, loading_msg.id, "Тут буде черга")
        .await?;
    Ok(())
}

pub async fn mixed(bot: Bot, msg: Message, _state: State) -> HandlerResult {
    let loading_msg = loading_message(&bot, msg.chat.id).await?;

    bot.edit_message_text(msg.chat.id, loading_msg.id, "Тут мішана буде черга")
        .await?;
    Ok(())
}

pub async fn priority_queue(bot: Bot, msg: Message, _state: State) -> HandlerResult {
    let loading_msg = loading_message(&bot, msg.chat.id).await?;

    bot.edit_message_text(msg.chat.id, loading_msg.id, "Тут буде черга з пріоритетом")
        .await?;
    Ok(())
}

async fn loading_message(bot: &Bot, chat_id: ChatId) -> anyhow::Result<Message> {
    let msg = bot
        .send_message(chat_id, "Створення нової черги...")
        .await?;

    Ok(msg)
}
