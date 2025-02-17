use sea_orm::{entity::*, query::*, DatabaseConnection};
use teloxide::types::UserId;

use crate::entities::{user_stats, users};
use crate::redis::RedisCache;
use crate::state::State;

use crate::entities::users::Entity as User;

pub async fn create_user_if_not_exists(
    state: &State,
    user: &teloxide::types::User,
    chat: &teloxide::types::Chat,
) -> anyhow::Result<()> {
    let conn: &DatabaseConnection = &state.db;

    tracing::debug!("Checking if user with id {} exists", user.id);

    if state.redis.get_user(user.id).is_ok() {
        tracing::debug!("User with id {} already exists in cache", user.id);
        return Ok(());
    }

    let existing_user = User::find()
        .filter(users::Column::AccountId.eq(user.id.to_string()))
        .one(conn)
        .await?;

    if let Some(existing_user) = existing_user {
        tracing::debug!("User with id {} already exists in database", user.id);
        state.redis.store_user(existing_user)?;
        return Ok(());
    }

    tracing::debug!("New user with id {} was created", user.id);
    let new_user = users::ActiveModel {
        username: Set(user.username.clone().unwrap_or_default()),
        account_id: Set(user.id.to_string()),
        chat_id: Set(chat.id.to_string()),
        name: Set(user.first_name.clone()),
        ..Default::default()
    }
    .insert(conn)
    .await?;

    state.redis.store_user(new_user.clone())?;

    user_stats::ActiveModel {
        user_id: Set(new_user.id),
        ..Default::default()
    }
    .insert(conn)
    .await?;

    Ok(())
}

pub async fn get_user_by_account_id(
    state: &State,
    user_id: UserId,
) -> anyhow::Result<users::Model> {
    User::find()
        .filter(users::Column::AccountId.eq(user_id.to_string()))
        .one(&state.db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("User not found"))
}

pub async fn get_user_by_id(state: &State, user_id: i32) -> anyhow::Result<users::Model> {
    User::find_by_id(user_id)
        .one(&state.db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("User not found"))
}
