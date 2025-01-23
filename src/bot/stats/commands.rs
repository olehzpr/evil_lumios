use crate::{bot::timetable::HandlerResult, State};
use teloxide::{prelude::Requester, types::Message, Bot};

pub async fn stats(bot: Bot, msg: Message, _state: State) -> HandlerResult {
    bot.send_message(msg.chat.id, "Stats command").await?;
    Ok(())
}

pub async fn me(bot: Bot, msg: Message, _state: State) -> HandlerResult {
    bot.send_message(msg.chat.id, "Me command").await?;
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
