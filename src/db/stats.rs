use anyhow::Context;
use sqlx::{PgPool, Row};
use teloxide::types::{ChatId, UserId};

use crate::models::stats::{FullStats, GambleModel, GroupMemberStat, GroupStats};
use crate::models::user::UserStatsModel;

pub async fn get_user_stats(pool: &PgPool, user_id: UserId) -> anyhow::Result<UserStatsModel> {
    let user_account_id_str = user_id.to_string();

    let stats = sqlx::query(
        r#"
        SELECT us.id, us.user_id, us.balance, us.daily_limit, us.daily_used
        FROM user_stats us
        JOIN users u ON us.user_id = u.id
        WHERE u.account_id = $1
        "#,
    )
    .bind(&user_account_id_str)
    .fetch_optional(pool)
    .await
    .context(format!(
        "Failed to query user stats for account_id: {}",
        user_account_id_str
    ))?
    .map(|row| UserStatsModel {
        id: row.get("id"),
        user_id: row.get("user_id"),
        balance: row.get("balance"),
        daily_limit: row.get("daily_limit"),
        daily_used: row.get("daily_used"),
    })
    .ok_or_else(|| {
        anyhow::anyhow!(
            "User stats not found for account_id: {}",
            user_account_id_str
        )
    })?;

    Ok(stats)
}

pub async fn get_full_me(pool: &PgPool, user_id: UserId) -> anyhow::Result<FullStats> {
    let user_account_id_str = user_id.to_string();

    let stats_row = sqlx::query(
        r#"
        SELECT us.id, us.user_id, us.balance, us.daily_limit, us.daily_used
        FROM user_stats us
        JOIN users u ON us.user_id = u.id
        WHERE u.account_id = $1
        "#,
    )
    .bind(user_account_id_str.clone())
    .fetch_optional(pool)
    .await
    .context("Failed to query user stats")?;

    let stats = stats_row
        .map(|row| UserStatsModel {
            id: row.get("id"),
            user_id: row.get("user_id"),
            balance: row.get("balance"),
            daily_limit: row.get("daily_limit"),
            daily_used: row.get("daily_used"),
        })
        .ok_or_else(|| {
            anyhow::anyhow!(
                "User stats not found for account_id: {}",
                user_account_id_str
            )
        })?;

    let gamble_rows = sqlx::query(
        r#"
        SELECT
            g.id,
            g.user_id,
            g.message_id,
            g.gamble_type,
            g.bet,
            g.change,
            g.is_win,
            g.created_at
        FROM gambles g
        JOIN users u ON g.user_id = u.id
        WHERE u.account_id = $1
        ORDER BY g.created_at ASC
        "#,
    )
    .bind(user_account_id_str)
    .fetch_all(pool)
    .await
    .context("Failed to query user gambles")?;

    let all_gambles = gamble_rows
        .into_iter()
        .map(|row| GambleModel {
            id: row.get("id"),
            user_id: row.get("user_id"),
            message_id: row.get("message_id"),
            gamble_type: row.get("gamble_type"),
            bet: row.get("bet"),
            change: row.get("change"),
            is_win: row.get("is_win"),
            created_at: row.get("created_at"),
        })
        .collect::<Vec<_>>();

    let mut total_won = 0;
    let mut total_lost = 0;
    let mut num_of_wins = 0;
    let mut num_of_losses = 0;
    let total_gambles = all_gambles.len() as i32;
    let mut longest_winning_streak = 0;
    let mut longest_losing_streak = 0;
    let mut current_streak = 0i32;
    let mut total_bet = 0.0;

    for gamble in all_gambles.iter() {
        if gamble.is_win {
            total_won += gamble.change.abs();
            num_of_wins += 1;
            if current_streak >= 0 {
                current_streak += 1;
            } else {
                longest_losing_streak = longest_losing_streak.max(current_streak.abs());
                current_streak = 1;
            }
            longest_winning_streak = longest_winning_streak.max(current_streak);
        } else {
            total_lost += gamble.change.abs();
            num_of_losses += 1;
            if current_streak <= 0 {
                current_streak -= 1;
            } else {
                longest_winning_streak = longest_winning_streak.max(current_streak);
                current_streak = -1;
            }
            longest_losing_streak = longest_losing_streak.max(current_streak.abs());
        }
        total_bet += gamble.bet as f32;
    }

    longest_winning_streak = longest_winning_streak.max(current_streak.max(0));
    longest_losing_streak = longest_losing_streak.max((-current_streak).max(0));

    let average_bet = if total_gambles > 0 {
        total_bet / total_gambles as f32
    } else {
        0.0
    };

    let full_stats = FullStats {
        user_id: stats.user_id,
        balance: stats.balance,
        daily_limit: stats.daily_limit,
        daily_used: stats.daily_used,
        total_won,
        total_lost,
        num_of_wins,
        num_of_losses,
        total_gambles,
        longest_winning_streak,
        longest_losing_streak,
        current_streak,
        average_bet,
    };

    Ok(full_stats)
}

pub async fn transfer_reaction_points(
    pool: &PgPool,
    sender_db_user_id: i32,
    receiver_db_user_id: i32,
    points: i32,
) -> anyhow::Result<()> {
    let mut tx = pool.begin().await.context("Failed to begin transaction")?;

    let sender_stats = sqlx::query(
        r#"
        SELECT id, user_id, balance, daily_limit, daily_used
        FROM user_stats
        WHERE user_id = $1
        "#,
    )
    .bind(sender_db_user_id)
    .fetch_optional(&mut *tx)
    .await
    .context("Failed to query sender stats")?
    .map(|row| UserStatsModel {
        id: row.get("id"),
        user_id: row.get("user_id"),
        balance: row.get("balance"),
        daily_limit: row.get("daily_limit"),
        daily_used: row.get("daily_used"),
    })
    .ok_or_else(|| anyhow::anyhow!("Sender stats not found for user_id: {}", sender_db_user_id))?;

    let receiver_stats = sqlx::query(
        r#"
        SELECT id, user_id, balance, daily_limit, daily_used
        FROM user_stats
        WHERE user_id = $1
        "#,
    )
    .bind(receiver_db_user_id)
    .fetch_optional(&mut *tx)
    .await
    .context("Failed to query receiver stats")?
    .map(|row| UserStatsModel {
        id: row.get("id"),
        user_id: row.get("user_id"),
        balance: row.get("balance"),
        daily_limit: row.get("daily_limit"),
        daily_used: row.get("daily_used"),
    })
    .ok_or_else(|| {
        anyhow::anyhow!(
            "Receiver stats not found for user_id: {}",
            receiver_db_user_id
        )
    })?;

    let available = sender_stats.daily_limit - sender_stats.daily_used;
    let actual = points.min(available);

    if actual <= 0 {
        tx.rollback()
            .await
            .context("Failed to rollback transaction")?;
        return Ok(());
    }

    sqlx::query(
        r#"
        UPDATE user_stats
        SET daily_used = daily_used + $1
        WHERE id = $2
        "#,
    )
    .bind(actual)
    .bind(sender_stats.id)
    .execute(&mut *tx)
    .await
    .context("Failed to update sender daily_used")?;

    sqlx::query(
        r#"
        UPDATE user_stats
        SET balance = balance + $1
        WHERE id = $2
        "#,
    )
    .bind(actual)
    .bind(receiver_stats.id)
    .execute(&mut *tx)
    .await
    .context("Failed to update receiver balance")?;

    tx.commit().await.context("Failed to commit transaction")?;
    Ok(())
}

pub async fn update_balance(pool: &PgPool, user_db_id: i32, change: i32) -> anyhow::Result<()> {
    let mut tx = pool.begin().await.context("Failed to begin transaction")?;

    let stats = sqlx::query(
        r#"
        SELECT id, user_id, balance, daily_limit, daily_used
        FROM user_stats
        WHERE user_id = $1
        "#,
    )
    .bind(user_db_id)
    .fetch_optional(&mut *tx)
    .await
    .context("Failed to query user stats for balance update")?
    .map(|row| UserStatsModel {
        id: row.get("id"),
        user_id: row.get("user_id"),
        balance: row.get("balance"),
        daily_limit: row.get("daily_limit"),
        daily_used: row.get("daily_used"),
    })
    .ok_or_else(|| anyhow::anyhow!("User stats not found for user_id: {}", user_db_id))?;

    sqlx::query(
        r#"
        UPDATE user_stats
        SET balance = balance + $1
        WHERE id = $2
        "#,
    )
    .bind(change)
    .bind(stats.id)
    .execute(&mut *tx)
    .await
    .context(format!(
        "Failed to update balance for user id: {}",
        user_db_id
    ))?;

    tx.commit().await.context("Failed to commit transaction")?;
    Ok(())
}

pub async fn get_group_stats(pool: &PgPool, chat_id: ChatId) -> anyhow::Result<GroupStats> {
    let chat_id_str = chat_id.to_string();

    let group_stats = sqlx::query(
        r#"
        SELECT
            u.username as username,
            us.balance as balance
        FROM user_stats us
        JOIN users u ON us.user_id = u.id
        WHERE u.chat_id = $1
        ORDER BY us.balance DESC
        "#,
    )
    .bind(chat_id_str.clone())
    .fetch_all(pool)
    .await
    .context(format!(
        "Failed to query group stats for chat_id: {}",
        chat_id_str
    ))?
    .into_iter()
    .map(|row| GroupMemberStat {
        username: row.get("username"),
        balance: row.get("balance"),
    })
    .collect();

    let group = sqlx::query(
        r#"
        SELECT title FROM chats WHERE chat_id = $1
        "#,
    )
    .bind(&chat_id_str)
    .fetch_optional(pool)
    .await
    .context(format!(
        "Failed to query chat title for chat_id: {}",
        chat_id_str
    ))?
    .and_then(|row| row.try_get("title").ok())
    .ok_or_else(|| anyhow::anyhow!("Chat not found for chat_id: {}", chat_id_str))?;

    Ok(GroupStats {
        group_name: group,
        stats: group_stats,
    })
}
