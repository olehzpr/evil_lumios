use anyhow::Context;
use sqlx::PgPool;
use teloxide::types::UserId;

use crate::models::user::{UserModel, UserStatsModel};
use crate::redis::RedisCache;
use crate::state::State;

pub async fn create_user_if_not_exists(
    state: &State,
    user: &teloxide::types::User,
    chat: &teloxide::types::Chat,
) -> anyhow::Result<()> {
    let pool: &PgPool = &state.db;
    let user_account_id_str = user.id.to_string();

    tracing::debug!(
        "Checking if user with account_id {} exists",
        user_account_id_str
    );

    if state.redis.get_user(user.id).is_ok() {
        tracing::debug!(
            "User with account_id {} already exists in cache",
            user_account_id_str
        );
        return Ok(());
    }

    let existing_user = sqlx::query_as::<_, UserModel>(
        r#"
        SELECT id, username, account_id, chat_id, name
        FROM users
        WHERE account_id = $1
        "#,
    )
    .bind(&user_account_id_str)
    .persistent(true)
    .fetch_optional(pool)
    .await
    .context("Failed to query user by account_id")?;

    if let Some(existing_user) = existing_user {
        tracing::debug!(
            "User with account_id {} already exists in database",
            user_account_id_str
        );
        state.redis.store_user(existing_user)?;
        return Ok(());
    }

    tracing::debug!(
        "New user with account_id {} will be created",
        user_account_id_str
    );

    let mut tx = pool.begin().await.context("Failed to begin transaction")?;

    let new_user = sqlx::query_as::<_, UserModel>(
        r#"
        INSERT INTO users (username, account_id, chat_id, name)
        VALUES ($1, $2, $3, $4)
        RETURNING id, username, account_id, chat_id, name
        "#,
    )
    .bind(user.username.clone().unwrap_or_default())
    .bind(&user_account_id_str)
    .bind(chat.id.to_string())
    .bind(user.first_name.clone())
    .persistent(true)
    .fetch_one(&mut *tx)
    .await
    .context("Failed to insert new user")?;

    tracing::debug!("New user created with database ID: {}", new_user.id);

    sqlx::query_as::<_, UserStatsModel>(
        r#"
        INSERT INTO user_stats (user_id, balance, daily_limit, daily_used)
        VALUES ($1, $2, $3, $4)
        RETURNING id, user_id, balance, daily_limit, daily_used
        "#,
    )
    .bind(new_user.id)
    .bind(1000)
    .bind(100)
    .bind(0)
    .persistent(true)
    .fetch_one(&mut *tx)
    .await
    .context("Failed to insert new user_stats")?;

    tracing::debug!("New user stats created for user ID: {}", new_user.id);

    tx.commit().await.context("Failed to commit transaction")?;
    state.redis.store_user(new_user)?;

    Ok(())
}

pub async fn get_user_by_account_id(state: &State, user_id: UserId) -> anyhow::Result<UserModel> {
    let pool: &PgPool = &state.db;
    let user_account_id_str = user_id.to_string();

    sqlx::query_as::<_, UserModel>(
        r#"
        SELECT id, username, account_id, chat_id, name
        FROM users
        WHERE account_id = $1
        "#,
    )
    .bind(&user_account_id_str)
    .persistent(true)
    .fetch_optional(pool)
    .await
    .context(format!(
        "Failed to query user by account_id: {}",
        user_account_id_str
    ))?
    .ok_or_else(|| anyhow::anyhow!("User with account_id {} not found", user_account_id_str))
}

pub async fn get_user_by_id(state: &State, user_id: i32) -> anyhow::Result<UserModel> {
    let pool: &PgPool = &state.db;

    sqlx::query_as::<_, UserModel>(
        r#"
        SELECT id, username, account_id, chat_id, name
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .context(format!("Failed to query user by id: {}", user_id))?
    .ok_or_else(|| anyhow::anyhow!("User with id {} not found", user_id))
}
