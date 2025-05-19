use anyhow::Context;
use sqlx::{PgPool, Row};

use crate::models::{gamble::GambleDto, stats::GambleModel};

pub async fn insert_gamble(pool: &PgPool, gamble: GambleDto) -> anyhow::Result<GambleModel> {
    let inserted_gamble = sqlx::query(
        r#"
        INSERT INTO gambles (user_id, message_id, is_win, change, bet, gamble_type)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, user_id, message_id, is_win, change, bet, gamble_type, created_at
        "#,
    )
    .bind(gamble.user_id)
    .bind(gamble.message_id.0)
    .bind(gamble.is_win)
    .bind(gamble.change)
    .bind(gamble.bet)
    .bind(String::from(gamble.gamble_type))
    .fetch_one(pool)
    .await
    .context(format!(
        "Failed to insert gamble for user_id: {}",
        gamble.user_id
    ))?;

    let inserted_gamble = GambleModel {
        id: inserted_gamble.get("id"),
        user_id: inserted_gamble.get("user_id"),
        message_id: inserted_gamble.get("message_id"),
        gamble_type: inserted_gamble.get("gamble_type"),
        bet: inserted_gamble.get("bet"),
        change: inserted_gamble.get("change"),
        is_win: inserted_gamble.get("is_win"),
        created_at: inserted_gamble.get("created_at"),
    };

    Ok(inserted_gamble)
}

pub async fn get_gamble_by_id(pool: &PgPool, id: i32) -> anyhow::Result<Option<GambleModel>> {
    let gamble = sqlx::query(
        r#"
        SELECT id, user_id, message_id, gamble_type, bet, change, is_win, created_at
        FROM gambles
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .context(format!("Failed to query gamble by id: {}", id))?;

    let gamble = gamble.map(|row| GambleModel {
        id: row.get("id"),
        user_id: row.get("user_id"),
        message_id: row.get("message_id"),
        gamble_type: row.get("gamble_type"),
        bet: row.get("bet"),
        change: row.get("change"),
        is_win: row.get("is_win"),
        created_at: row.get("created_at"),
    });

    Ok(gamble)
}
