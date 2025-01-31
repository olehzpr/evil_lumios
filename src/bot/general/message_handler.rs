use teloxide::{prelude::Requester, types::Message, Bot};

use crate::{bot::handler::HandlerResult, redis::RedisCache, state::State};

pub async fn handler(bot: Bot, msg: Message, state: State) -> HandlerResult {
    if let (Some(text), Some(reply)) = (msg.text(), msg.reply_to_message()) {
        let username = reply.from.as_ref().unwrap().username.as_ref().unwrap();
        let bot_username = bot.get_me().await?.user.username.unwrap();
        if username == &bot_username && text.contains("іді нахуй") {
            bot.send_message(msg.chat.id, "сам іді нахуй").await?;
        }
    }

    state.redis.store_message(msg)?;

    Ok(())
}
