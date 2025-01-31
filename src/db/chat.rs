use diesel::{ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl};
use teloxide::types::ChatId;

use crate::schema::{self};
use crate::state::{CacheValue, State};

use super::{
    models::{Chat, NewChat},
    StateWithConnection,
};

pub async fn create_chat_if_not_exists(
    state: &State,
    chat: &teloxide::types::Chat,
) -> anyhow::Result<()> {
    let conn = &mut state.conn().await;

    tracing::debug!("Checking if chat with id {} exists", chat.id);
    let key = format!("chat_{}", chat.id);
    if state.cache.get(&key).is_some() {
        tracing::debug!("Chat with id {} already exists in cache", chat.id);
        return Ok(());
    }

    let existing_chat = schema::chats::table
        .filter(schema::chats::chat_id.eq(chat.id.to_string()))
        .first::<Chat>(conn)
        .optional()?;

    if existing_chat.is_some() {
        tracing::debug!("Chat with id {} already exists in database", chat.id);
        state.cache.insert(key, CacheValue::Chat(chat.id));
        return Ok(());
    }

    tracing::debug!("New chat with id {} was created", chat.id);

    diesel::insert_into(crate::schema::chats::table)
        .values(NewChat {
            chat_id: &chat.id.to_string(),
            group_id: None,
            title: &chat.title().unwrap_or_default(),
            description: chat.description(),
        })
        .execute(conn)?;

    state.cache.insert(key, CacheValue::Chat(chat.id));

    Ok(())
}

pub async fn get_chats(conn: &mut PgConnection) -> anyhow::Result<Vec<ChatId>> {
    schema::timetables::table
        .select(schema::timetables::chat_id)
        .distinct()
        .load::<String>(conn)?
        .into_iter()
        .map(|id| id.parse().map(ChatId))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| anyhow::anyhow!(e))
}
