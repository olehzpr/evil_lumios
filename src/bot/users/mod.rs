use diesel::prelude::*;
use evil_lumios::State;

use crate::db::{
    connection,
    models::{NewUser, User},
};

pub async fn create_user_if_not_exists(
    state: &State,
    user: &Option<teloxide::types::User>,
    chat: &teloxide::types::Chat,
) {
    if user.is_none() {
        return;
    }
    let user = user.as_ref().unwrap();
    let conn = &mut connection(&state).await;
    let existing_user = diesel::sql_query("SELECT * FROM users WHERE id = $1")
        .bind::<diesel::sql_types::Integer, _>(user.id.0 as i32)
        .get_result::<User>(conn)
        .optional();
    if let Err(e) = existing_user {
        eprintln!("Failed to get chat: {:?}", e);
        return;
    }
    if existing_user.ok().unwrap().is_some() {
        return;
    }
    _ = diesel::insert_into(crate::schema::users::table)
        .values(NewUser {
            username: user.username.as_ref().unwrap_or(&String::new()),
            account_id: &user.id.to_string(),
            chat_id: &chat.id.to_string(),
        })
        .execute(conn);
}
