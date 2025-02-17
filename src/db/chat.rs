use sea_orm::{entity::*, query::*, DatabaseConnection};
use teloxide::types::ChatId;

use crate::entities::chats;
use crate::redis::RedisCache;
use crate::state::State;

use crate::entities::chats::Entity as Chat;

pub async fn create_chat_if_not_exists(
    state: &State,
    chat: &teloxide::types::Chat,
) -> anyhow::Result<()> {
    let conn: &DatabaseConnection = &state.db;

    tracing::debug!("Checking if chat with id {} exists", chat.id);

    if state.redis.get_chat(chat.id).is_ok() {
        tracing::debug!("Chat with id {} already exists in cache", chat.id);
        return Ok(());
    }

    let existing_chat = Chat::find()
        .filter(chats::Column::ChatId.eq(chat.id.to_string()))
        .one(conn)
        .await?;

    if let Some(existing_chat) = existing_chat {
        tracing::debug!("Chat with id {} already exists in database", chat.id);
        state.redis.store_chat(existing_chat)?;
        return Ok(());
    }

    tracing::debug!("New chat with id {} was created", chat.id);
    let new_chat = chats::ActiveModel {
        chat_id: Set(chat.id.to_string()),
        group_id: Set(None),
        title: Set(chat.title().unwrap_or_default().to_owned()),
        description: Set(chat.description().map(|desc| desc.to_string())),
        ..Default::default()
    }
    .insert(conn)
    .await?;

    state.redis.store_chat(new_chat)?;

    Ok(())
}

pub async fn get_chat_ids(conn: &DatabaseConnection) -> anyhow::Result<Vec<ChatId>> {
    let chats = Chat::find().all(conn).await?;
    let chat_ids = chats
        .into_iter()
        .map(|chat| ChatId(chat.chat_id.parse().unwrap()))
        .collect();
    Ok(chat_ids)
}
