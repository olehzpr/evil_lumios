use crate::{bot::timetable::HandlerResult, State};
use teloxide::{prelude::Requester, types::Message, Bot};

pub async fn queue(bot: Bot, msg: Message, _state: State) -> HandlerResult {
    bot.send_message(msg.chat.id, "Queue command").await?;
    Ok(())
}

pub async fn mixed(bot: Bot, msg: Message, _state: State) -> HandlerResult {
    bot.send_message(msg.chat.id, "Mixed Queue command").await?;
    Ok(())
}
