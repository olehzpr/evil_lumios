use teloxide::{
    prelude::Requester,
    types::{CallbackQuery, MessageId, UserId},
    Bot,
};

use crate::state::State;

use super::handler::HandlerResult;

pub mod stats;

pub enum Callback {
    ShowFullStats(MessageId, UserId),
}
impl Callback {
    pub fn from_str(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('_').collect();
        match parts.as_slice() {
            ["show-full-stats", user_id, message_id] => {
                let message_id = message_id.parse().ok()?;
                let user_id = user_id.parse().ok()?;
                Some(Callback::ShowFullStats(
                    MessageId(message_id),
                    UserId(user_id),
                ))
            }
            _ => None,
        }
    }
}
pub async fn handle_callback(bot: Bot, state: State, q: CallbackQuery) -> HandlerResult {
    if q.data.is_none() {
        bot.answer_callback_query(q.id).await?;
        return Ok(());
    }
    match Callback::from_str(q.data.as_ref().unwrap()) {
        Some(Callback::ShowFullStats(message_id, user_id)) => {
            stats::show_full_stats(bot, state, message_id, user_id, q).await?;
        }
        None => {
            bot.answer_callback_query(q.id).await?;
        }
    }
    Ok(())
}
