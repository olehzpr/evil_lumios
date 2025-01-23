use crate::state::State;
use diesel::prelude::*;

use crate::db::{
    models::{Chat, NewChat},
    StateWithConnection,
};

pub async fn create_chat_if_not_exists(state: &State, chat: &teloxide::types::Chat) {
    let conn = &mut state.conn().await;
    let existing_chat = diesel::sql_query("SELECT * FROM chats WHERE id = $1")
        .bind::<diesel::sql_types::Integer, _>(chat.id.0 as i32)
        .get_result::<Chat>(conn)
        .optional();
    if let Err(e) = existing_chat {
        eprintln!("Failed to get chat: {:?}", e);
        return;
    }
    if existing_chat.ok().unwrap().is_some() {
        return;
    }
    _ = diesel::insert_into(crate::schema::chats::table)
        .values(NewChat {
            chat_id: &chat.id.to_string(),
            group_id: None,
            title: &chat.title().unwrap_or_default(),
            description: chat.description(),
        })
        .execute(conn);
}
