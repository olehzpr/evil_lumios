use teloxide::{prelude::Requester, types::Message, Bot};

use crate::{
    bot::handler::HandlerResult,
    state::{CacheValue, State},
};

pub async fn handler(bot: Bot, msg: Message, state: State) -> HandlerResult {
    if let (Some(text), Some(reply)) = (msg.text(), msg.reply_to_message()) {
        let username = reply.from.as_ref().unwrap().username.as_ref().unwrap();
        let bot_username = bot.get_me().await?.user.username.unwrap();
        if username == &bot_username && text.contains("іді нахуй") {
            bot.send_message(msg.chat.id, "сам іді нахуй").await?;
        }
    }
    let mut fifo_cache = state.fifo_cache.lock().unwrap();
    if fifo_cache.messages.len() as u64 >= 100 {
        let front = fifo_cache.messages.pop_front();
        if let Some(front) = front {
            state.cache.remove(format!("message:{}", front).as_str());
        }
    }
    fifo_cache.messages.push_back(msg.id);
    state
        .cache
        .insert(format!("message:{}", msg.id), CacheValue::Message(msg));
    Ok(())
}
