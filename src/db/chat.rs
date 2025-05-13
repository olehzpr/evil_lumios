use anyhow::Context;
use sqlx::{PgPool, Row};
use teloxide::types::ChatId;

use crate::models::chat::ChatModel;
use crate::redis::RedisCache;
use crate::state::State;

pub async fn create_chat_if_not_exists(
    state: &State,
    chat: &teloxide::types::Chat,
) -> anyhow::Result<()> {
    let pool: &PgPool = &state.db;
    let chat_id_str = chat.id.to_string();

    tracing::debug!("Checking if chat with id {} exists", chat_id_str);

    if state.redis.get_chat(chat.id).is_ok() {
        tracing::debug!("Chat with id {} already exists in cache", chat_id_str);
        return Ok(());
    }

    let existing_chat = sqlx::query(
        r#"
        SELECT id, chat_id, group_id, title, description
        FROM chats
        WHERE chat_id = $1
        "#,
    )
    .bind(&chat_id_str)
    .fetch_optional(pool)
    .await
    .context(format!("Failed to query chat by chat_id: {}", chat_id_str))?;

    let existing_chat = existing_chat.map(|row| ChatModel {
        id: row.get("id"),
        chat_id: row.get("chat_id"),
        group_id: row.get("group_id"),
        title: row.get("title"),
        description: row.get("description"),
    });

    if let Some(existing_chat) = existing_chat {
        tracing::debug!("Chat with id {} already exists in database", chat_id_str);

        state.redis.store_chat(existing_chat)?;
        return Ok(());
    }

    tracing::debug!("New chat with id {} will be created", chat_id_str);

    let new_chat = sqlx::query(
        r#"
        INSERT INTO chats (chat_id, group_id, title, description)
        VALUES ($1, $2, $3, $4)
        RETURNING id, chat_id, group_id, title, description
        "#,
    )
    .bind(&chat_id_str)
    .bind(None::<String>)
    .bind(chat.title().unwrap_or_default().to_owned())
    .bind(chat.description().map(|desc| desc.to_string()))
    .fetch_one(pool)
    .await
    .context(format!(
        "Failed to insert new chat with id: {}",
        chat_id_str
    ))?;

    let new_chat = ChatModel {
        id: new_chat.get("id"),
        chat_id: new_chat.get("chat_id"),
        group_id: new_chat.get("group_id"),
        title: new_chat.get("title"),
        description: new_chat.get("description"),
    };

    tracing::debug!("New chat created with database ID: {}", new_chat.id);

    state.redis.store_chat(new_chat)?;

    Ok(())
}

pub async fn get_chat_ids(pool: &PgPool) -> anyhow::Result<Vec<ChatId>> {
    let chats = sqlx::query(
        r#"
        SELECT id, chat_id, group_id, title, description
        FROM chats
        "#,
    )
    .fetch_all(pool)
    .await
    .context("Failed to query all chats")?;

    let chat_ids = chats
        .into_iter()
        .map(|row| ChatModel {
            id: row.get("id"),
            chat_id: row.get("chat_id"),
            group_id: row.get("group_id"),
            title: row.get("title"),
            description: row.get("description"),
        })
        .filter_map(|chat| chat.chat_id.parse::<i64>().ok().map(ChatId))
        .collect();

    Ok(chat_ids)
}
