use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use teloxide::types::UserId;

use crate::redis::RedisCache;
use crate::schema::{self};
use crate::state::State;

use super::models::{NewUser, User};
use super::StateWithConnection;

pub async fn create_user_if_not_exists(
    state: &State,
    user: &teloxide::types::User,
    chat: &teloxide::types::Chat,
) -> anyhow::Result<()> {
    let conn = &mut state.conn().await;

    tracing::debug!("Checking if user with id {} exists", user.id);

    if state.redis.get_user(user.id).is_ok() {
        tracing::debug!("User with id {} already exists in cache", user.id);
        return Ok(());
    }

    let existing_user = schema::users::table
        .filter(schema::users::account_id.eq(user.id.to_string()))
        .first::<User>(conn)
        .optional()?;

    if let Some(existing_user) = existing_user {
        tracing::debug!("User with id {} already exists in database", user.id);
        state.redis.store_user(existing_user)?;
        return Ok(());
    }

    tracing::debug!("New user with id {} was created", user.id);
    let new_user = diesel::insert_into(crate::schema::users::table)
        .values(NewUser {
            username: user.username.as_ref().unwrap_or(&String::new()),
            account_id: &user.id.to_string(),
            chat_id: &chat.id.to_string(),
            name: &user.first_name,
        })
        .get_result::<User>(conn)?;

    state.redis.store_user(new_user.clone())?;

    diesel::insert_into(schema::user_stats::table)
        .values(crate::db::models::NewUserStats {
            user_id: new_user.id,
        })
        .execute(conn)?;

    Ok(())
}

pub async fn get_user_by_account_id(state: &State, user_id: UserId) -> anyhow::Result<User> {
    schema::users::table
        .filter(schema::users::account_id.eq(user_id.to_string()))
        .first::<User>(&mut state.conn().await)
        .map_err(|e| anyhow::anyhow!(e))
}

pub async fn get_user_by_id(state: &State, user_id: i32) -> anyhow::Result<User> {
    schema::users::table
        .find(user_id)
        .first::<User>(&mut state.conn().await)
        .map_err(|e| anyhow::anyhow!(e))
}
