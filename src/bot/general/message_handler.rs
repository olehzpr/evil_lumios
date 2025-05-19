use teloxide::{
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{Message, ReplyParameters},
    Bot,
};

use crate::{
    bot::{handler::HandlerResult, ui::utils::adapt_for_markdown},
    clients::gemini::send_to_gemini,
    redis::RedisCache,
    state::State,
};

pub async fn handler(bot: Bot, msg: Message, state: State) -> HandlerResult {
    handle_fuck_off(&bot, &msg).await?;

    handle_gemini_mention(&bot, &msg).await?;

    state.redis.store_message(msg)?;

    Ok(())
}

async fn handle_gemini_mention(bot: &Bot, msg: &Message) -> HandlerResult {
    let text = match msg.text() {
        Some(t) => t,
        None => return Ok(()),
    };

    let bot_username = bot.get_me().await?.user.username.unwrap();
    let mentioned = text.contains(&bot_username);

    if !mentioned {
        return Ok(());
    }

    let cleaned_text = text
        .replace(&format!("@{}", bot_username), "")
        .trim()
        .to_string();
    if cleaned_text.is_empty() {
        return Ok(());
    }

    let response = send_to_gemini(&cleaned_text).await?;
    bot.send_message(msg.chat.id, adapt_for_markdown(&response))
        .reply_parameters(ReplyParameters::new(msg.id))
        .await?;

    Ok(())
}

async fn handle_fuck_off(bot: &Bot, msg: &Message) -> HandlerResult {
    if let (Some(text), Some(reply)) = (msg.text(), msg.reply_to_message()) {
        let username = reply.from.as_ref().unwrap().username.as_ref().unwrap();
        let bot_username = bot.get_me().await?.user.username.unwrap();
        if username == &bot_username && text.contains("іді нахуй") {
            bot.send_message(msg.chat.id, "сам іді нахуй").await?;
        }
    }

    Ok(())
}
