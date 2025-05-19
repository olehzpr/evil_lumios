pub mod commands;

use async_trait::async_trait;
use teloxide::{
    payloads::EditMessageTextSetters,
    prelude::{Request, Requester},
    types::{ChatId, InlineKeyboardMarkup, MessageId, ParseMode},
    Bot,
};

use crate::models::queue::{QueueModel, QueueUserWithUserModel};

use super::{ui, utils::reply_markup_builder::ReplyMarkupBuilder};

pub trait QueueMarkupExt {
    fn regular_queue_markup(queue_id: i32) -> InlineKeyboardMarkup;
    fn mixed_queue_markup(queue_id: i32, is_mixed: bool) -> InlineKeyboardMarkup;
    fn priority_queue_markup(queue_id: i32) -> InlineKeyboardMarkup;
}

impl QueueMarkupExt for ReplyMarkupBuilder {
    fn regular_queue_markup(queue_id: i32) -> InlineKeyboardMarkup {
        ReplyMarkupBuilder::new()
            .button_row(vec![
                ("Join 📡", format!("join-queue_{}", queue_id)),
                ("Leave 🔄", format!("leave-queue_{}", queue_id)),
            ])
            .button_row(vec![
                ("Delete ❌", format!("delete-queue_{}", queue_id)),
                ("Notify 📢", format!("notify-queue_{}", queue_id)),
            ])
            .build()
    }

    fn mixed_queue_markup(queue_id: i32, is_mixed: bool) -> InlineKeyboardMarkup {
        let mut markup = ReplyMarkupBuilder::new()
            .button_row(vec![
                ("Join 📡", format!("join-queue_{}", queue_id)),
                ("Leave 🔄", format!("leave-queue_{}", queue_id)),
            ])
            .button_row(vec![
                ("Delete ❌", format!("delete-queue_{}", queue_id)),
                ("Notify 📢", format!("notify-queue_{}", queue_id)),
            ]);

        if !is_mixed {
            markup = markup.single_button("Shuffle 🔀", format!("shuffle-queue_{}", queue_id));
        }

        markup.build()
    }

    fn priority_queue_markup(queue_id: i32) -> InlineKeyboardMarkup {
        ReplyMarkupBuilder::new()
            .button_row(vec![
                ("Join 📡", format!("join-queue_{}", queue_id)),
                ("Leave 🔄", format!("leave-queue_{}", queue_id)),
            ])
            .button_row(vec![
                ("Done ✅", format!("done-queue_{}", queue_id)),
                ("Skip ⏩", format!("skip-queue_{}", queue_id)),
            ])
            .button_row(vec![
                ("Freeze ❄️", format!("freeze-queue_{}", queue_id)),
                ("Notify 📢", format!("notify-queue_{}", queue_id)),
            ])
            .build()
    }
}

#[async_trait]
pub trait QueueMessages {
    async fn edit_queue(&self, queue: QueueModel, users: Vec<QueueUserWithUserModel>);
}

#[async_trait]
impl QueueMessages for Bot {
    async fn edit_queue(&self, queue: QueueModel, users: Vec<QueueUserWithUserModel>) {
        let content = if queue.is_priority {
            ui::queue::priority_queue(&queue, users)
        } else {
            ui::queue::regular_queue(&queue, users)
        };

        let markup = if queue.is_mixed.is_some() {
            ReplyMarkupBuilder::mixed_queue_markup(queue.id, queue.is_mixed.unwrap_or(false))
        } else if queue.is_priority {
            ReplyMarkupBuilder::priority_queue_markup(queue.id)
        } else {
            ReplyMarkupBuilder::regular_queue_markup(queue.id)
        };

        let result = self
            .edit_message_text(ChatId(queue.chat_id), MessageId(queue.message_id), content)
            .reply_markup(markup)
            .parse_mode(ParseMode::MarkdownV2)
            .send()
            .await;

        if let Err(e) = result {
            tracing::error!("Failed to edit regular/mixed queue: {:?}", e);
        }
    }
}
