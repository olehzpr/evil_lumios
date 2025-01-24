use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};

use crate::schema::{self};
use crate::state::{CacheValue, State};

use super::models::{NewUser, User};
use super::StateWithConnection;

pub async fn create_user_if_not_exists(
    state: &State,
    user: &teloxide::types::User,
    chat: &teloxide::types::Chat,
) -> anyhow::Result<()> {
    let conn = &mut state.conn().await;

    tracing::debug!("Checking if user with id {} exists", user.id);
    let key = format!("user_{}", user.id);
    if state.cache.get(&key).is_some() {
        tracing::debug!("User with id {} already exists in cache", user.id);
        return Ok(());
    }

    let existing_user = schema::users::table
        .filter(schema::users::account_id.eq(user.id.to_string()))
        .first::<User>(conn)
        .optional()?;

    if existing_user.is_some() {
        tracing::debug!("User with id {} already exists in database", user.id);
        state.cache.insert(key, CacheValue::User(user.id));
        return Ok(());
    }

    tracing::debug!("New user with id {} was created", user.id);

    diesel::insert_into(crate::schema::users::table)
        .values(NewUser {
            username: user.username.as_ref().unwrap_or(&String::new()),
            account_id: &user.id.to_string(),
            chat_id: &chat.id.to_string(),
            name: &user.first_name,
        })
        .execute(conn)?;

    Ok(())
}
