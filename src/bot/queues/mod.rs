use async_trait::async_trait;
use teloxide::{
    payloads::EditMessageTextSetters,
    prelude::{Request, Requester},
    types::{ChatId, InlineKeyboardButton, InlineKeyboardMarkup, MessageId},
    Bot,
};

pub mod commands;

#[async_trait]
pub trait QueueMessages {
    async fn edit_regular_queue(
        &self,
        chat_id: ChatId,
        message_id: MessageId,
        queue_id: i32,
        content: &str,
    );
    async fn edit_mixed_queue(
        &self,
        chat_id: ChatId,
        message_id: MessageId,
        queue_id: i32,
        content: &str,
    );
    async fn edit_priority_queue(
        &self,
        chat_id: ChatId,
        message_id: MessageId,
        queue_id: i32,
        content: &str,
    );
}

#[async_trait]
impl QueueMessages for Bot {
    async fn edit_regular_queue(
        &self,
        chat_id: ChatId,
        message_id: MessageId,
        queue_id: i32,
        content: &str,
    ) {
        let _ = self
            .edit_message_text(chat_id, message_id, content)
            .reply_markup(InlineKeyboardMarkup::new(vec![
                vec![
                    InlineKeyboardButton::callback("Join 📡", format!("join-queue_{}", queue_id)),
                    InlineKeyboardButton::callback("Leave 🔄", format!("leave-queue_{}", queue_id)),
                ],
                vec![
                    InlineKeyboardButton::callback(
                        "Delete ❌",
                        format!("delete-queue_{}", queue_id),
                    ),
                    InlineKeyboardButton::callback(
                        "Notify 📢",
                        format!("notify-queue_{}", queue_id),
                    ),
                ],
            ]))
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to edit regular queue: {:?}", e);
            });
    }

    async fn edit_mixed_queue(
        &self,
        chat_id: ChatId,
        message_id: MessageId,
        queue_id: i32,
        content: &str,
    ) {
        let _ = self
            .edit_message_text(chat_id, message_id, content)
            .reply_markup(InlineKeyboardMarkup::new(vec![
                vec![
                    InlineKeyboardButton::callback("Join 📡", format!("join-queue_{}", queue_id)),
                    InlineKeyboardButton::callback("Leave 🔄", format!("leave-queue_{}", queue_id)),
                ],
                vec![
                    InlineKeyboardButton::callback(
                        "Delete ❌",
                        format!("delete-queue_{}", queue_id),
                    ),
                    InlineKeyboardButton::callback(
                        "Notify 📢",
                        format!("notify-queue_{}", queue_id),
                    ),
                ],
                vec![InlineKeyboardButton::callback(
                    "Shuffle 🔀",
                    format!("shuffle-queue_{}", queue_id),
                )],
            ]))
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to edit regular queue: {:?}", e);
            });
    }

    async fn edit_priority_queue(
        &self,
        chat_id: ChatId,
        message_id: MessageId,
        queue_id: i32,
        content: &str,
    ) {
        let _ = self
            .edit_message_text(chat_id, message_id, content)
            .reply_markup(InlineKeyboardMarkup::new(vec![
                vec![
                    InlineKeyboardButton::callback("Join 📡", format!("join-queue_{}", queue_id)),
                    InlineKeyboardButton::callback("Leave 🔄", format!("leave-queue_{}", queue_id)),
                ],
                vec![
                    InlineKeyboardButton::callback(
                        "Freeze ❄️",
                        format!("freeze-queue_{}", queue_id),
                    ),
                    InlineKeyboardButton::callback("Skip ⏩", format!("skip-queue_{}", queue_id)),
                ],
                vec![
                    InlineKeyboardButton::callback(
                        "Delete ❌",
                        format!("delete-queue_{}", queue_id),
                    ),
                    InlineKeyboardButton::callback(
                        "Notify 📢",
                        format!("notify-queue_{}", queue_id),
                    ),
                ],
            ]))
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to edit regular queue: {:?}", e);
            });
    }
}
