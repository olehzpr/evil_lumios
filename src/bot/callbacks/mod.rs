use teloxide::{
    prelude::Requester,
    types::{CallbackQuery, MessageId, UserId},
    Bot,
};

use crate::state::State;

use super::handler::HandlerResult;

pub mod queue_callbacks;
pub mod stats_callbacks;

pub enum Callback {
    ShowFullStats(MessageId, UserId),
    JoinQueue(i32),
    LeaveQueue(i32),
    DeleteQueue(i32),
    NotifyQueue(i32),
    ShuffleQueue(i32),
    FreezeQueue(i32),
    SkipQueue(i32),
    DoneQueue(i32),
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
            ["join-queue", queue_id] => {
                let queue_id = queue_id.parse().ok()?;
                Some(Callback::JoinQueue(queue_id))
            }
            ["leave-queue", queue_id] => {
                let queue_id = queue_id.parse().ok()?;
                Some(Callback::LeaveQueue(queue_id))
            }
            ["delete-queue", queue_id] => {
                let queue_id = queue_id.parse().ok()?;
                Some(Callback::DeleteQueue(queue_id))
            }
            ["notify-queue", queue_id] => {
                let queue_id = queue_id.parse().ok()?;
                Some(Callback::NotifyQueue(queue_id))
            }
            ["shuffle-queue", queue_id] => {
                let queue_id = queue_id.parse().ok()?;
                Some(Callback::ShuffleQueue(queue_id))
            }
            ["freeze-queue", queue_id] => {
                let queue_id = queue_id.parse().ok()?;
                Some(Callback::FreezeQueue(queue_id))
            }
            ["skip-queue", queue_id] => {
                let queue_id = queue_id.parse().ok()?;
                Some(Callback::SkipQueue(queue_id))
            }
            ["done-queue", queue_id] => {
                let queue_id = queue_id.parse().ok()?;
                Some(Callback::DoneQueue(queue_id))
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
            stats_callbacks::show_full_stats(bot, state, message_id, user_id, q).await?;
        }
        Some(Callback::JoinQueue(queue_id)) => {
            queue_callbacks::join_queue(bot, state, queue_id, q).await?;
        }
        Some(Callback::LeaveQueue(queue_id)) => {
            queue_callbacks::leave_queue(bot, state, queue_id, q).await?;
        }
        Some(Callback::DeleteQueue(queue_id)) => {
            queue_callbacks::delete_queue(bot, state, queue_id, q).await?;
        }
        Some(Callback::NotifyQueue(queue_id)) => {
            queue_callbacks::notify_queue(bot, state, queue_id, q).await?;
        }
        Some(Callback::ShuffleQueue(queue_id)) => {
            queue_callbacks::shuffle_queue(bot, state, queue_id, q).await?;
        }
        Some(Callback::FreezeQueue(queue_id)) => {
            queue_callbacks::freeze_queue(bot, state, queue_id, q).await?;
        }
        Some(Callback::SkipQueue(queue_id)) => {
            queue_callbacks::skip_queue(bot, state, queue_id, q).await?;
        }
        Some(Callback::DoneQueue(queue_id)) => {
            queue_callbacks::done_queue(bot, state, queue_id, q).await?;
        }
        None => {
            bot.answer_callback_query(q.id).await?;
        }
    }
    Ok(())
}
