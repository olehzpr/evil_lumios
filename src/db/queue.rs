use anyhow::{bail, Context};
use rand::seq::SliceRandom;
use rand::{rngs::StdRng, SeedableRng};
use sqlx::{PgPool, Row};
use teloxide::types::{ChatId, MessageId};

use crate::models::queue::{QueueModel, QueueUserModel, QueueUserWithUserModel};

pub async fn create_queue(
    pool: &PgPool,
    title: &String,
    chat_id: ChatId,
    message_id: MessageId,
    is_mixed: Option<bool>,
    is_priority: bool,
) -> anyhow::Result<QueueModel> {
    let new_queue = sqlx::query(
        r#"
        INSERT INTO queues (title, chat_id, message_id, is_mixed, is_priority)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, title, chat_id, message_id, is_mixed, is_priority, is_deleted, created_at
        "#,
    )
    .bind(title)
    .bind(chat_id.to_string())
    .bind(message_id.to_string())
    .bind(is_mixed)
    .bind(is_priority)
    .fetch_one(pool)
    .await
    .context("Failed to insert new queue")?;

    let new_queue = QueueModel {
        id: new_queue.get("id"),
        title: new_queue.get("title"),
        chat_id: new_queue.get("chat_id"),
        message_id: new_queue.get("message_id"),
        is_mixed: new_queue.get("is_mixed"),
        is_priority: new_queue.get("is_priority"),
        is_deleted: new_queue.get("is_deleted"),
        created_at: new_queue.get("created_at"),
    };

    Ok(new_queue)
}

pub async fn get_all_queues(pool: &PgPool, chat_id: ChatId) -> anyhow::Result<Vec<QueueModel>> {
    let chat_id_str = chat_id.to_string();

    let queues = sqlx::query(
        r#"
        SELECT id, title, chat_id, message_id, is_mixed, is_priority, is_deleted, created_at
        FROM queues
        WHERE chat_id = $1 AND is_deleted = FALSE
        "#,
    )
    .bind(&chat_id_str)
    .fetch_all(pool)
    .await
    .context("Failed to query all queues")?
    .into_iter()
    .map(|row| QueueModel {
        id: row.get("id"),
        title: row.get("title"),
        chat_id: row.get("chat_id"),
        message_id: row.get("message_id"),
        is_mixed: row.get("is_mixed"),
        is_priority: row.get("is_priority"),
        is_deleted: row.get("is_deleted"),
        created_at: row.get("created_at"),
    })
    .collect();

    Ok(queues)
}

pub async fn delete_queue(pool: &PgPool, queue_id: i32) -> anyhow::Result<()> {
    let result = sqlx::query(
        r#"
        UPDATE queues
        SET is_deleted = TRUE
        WHERE id = $1 AND is_deleted = FALSE
        "#,
    )
    .bind(queue_id)
    .execute(pool)
    .await
    .context("Failed to soft delete queue")?;

    if result.rows_affected() == 0 {
        bail!("Queue not found or already deleted");
    }

    Ok(())
}

pub async fn get_queue(
    pool: &PgPool,
    chat_id: ChatId,
    message_id: MessageId,
) -> anyhow::Result<QueueModel> {
    let chat_id_str = chat_id.to_string();
    let message_id_str = message_id.to_string();

    let queue = sqlx::query(
        r#"
        SELECT id, title, chat_id, message_id, is_mixed, is_priority, is_deleted, created_at
        FROM queues
        WHERE chat_id = $1 AND message_id = $2 AND is_deleted = FALSE
        "#,
    )
    .bind(&chat_id_str)
    .bind(&message_id_str)
    .fetch_optional(pool)
    .await
    .context("Failed to query queue")?
    .map(|row| QueueModel {
        id: row.get("id"),
        title: row.get("title"),
        chat_id: row.get("chat_id"),
        message_id: row.get("message_id"),
        is_mixed: row.get("is_mixed"),
        is_priority: row.get("is_priority"),
        is_deleted: row.get("is_deleted"),
        created_at: row.get("created_at"),
    })
    .ok_or_else(|| anyhow::anyhow!("Queue not found"))?;

    Ok(queue)
}

pub async fn get_queue_by_id(pool: &PgPool, queue_id: i32) -> anyhow::Result<QueueModel> {
    let queue = sqlx::query(
        r#"
        SELECT id, title, chat_id, message_id, is_mixed, is_priority, is_deleted, created_at
        FROM queues
        WHERE id = $1 AND is_deleted = FALSE
        "#,
    )
    .bind(queue_id)
    .fetch_optional(pool)
    .await
    .context("Failed to query queue by id")?
    .map(|row| QueueModel {
        id: row.get("id"),
        title: row.get("title"),
        chat_id: row.get("chat_id"),
        message_id: row.get("message_id"),
        is_mixed: row.get("is_mixed"),
        is_priority: row.get("is_priority"),
        is_deleted: row.get("is_deleted"),
        created_at: row.get("created_at"),
    })
    .ok_or_else(|| anyhow::anyhow!("Queue not found"))?;

    Ok(queue)
}

pub async fn shuffle_queue(pool: &PgPool, queue_id: i32) -> anyhow::Result<()> {
    let mut tx = pool
        .begin()
        .await
        .context("Failed to begin transaction for shuffle")?;

    let mut all_users = sqlx::query(
        r#"
        SELECT id, position, priority, is_frozen, queue_id, user_id
        FROM queue_users
        WHERE queue_id = $1
        ORDER BY position
        "#,
    )
    .bind(queue_id)
    .fetch_all(&mut *tx)
    .await
    .context("Failed to fetch queue users for shuffle")?
    .into_iter()
    .map(|row| QueueUserModel {
        id: row.get("id"),
        position: row.get("position"),
        priority: row.get("priority"),
        is_frozen: row.get("is_frozen"),
        queue_id: row.get("queue_id"),
        user_id: row.get("user_id"),
    })
    .collect::<Vec<_>>();

    if all_users.is_empty() {
        tx.rollback()
            .await
            .context("Failed to rollback empty shuffle transaction")?;
        return Ok(());
    }

    let mut rng = StdRng::from_entropy();
    all_users.shuffle(&mut rng);

    for (i, user) in all_users.iter().enumerate() {
        let new_position = i as i32 + 1;
        sqlx::query(
            r#"
            UPDATE queue_users
            SET position = $1
            WHERE id = $2
            "#,
        )
        .bind(new_position)
        .bind(user.id)
        .execute(&mut *tx)
        .await
        .context("Failed to update queue user position")?;
    }

    tx.commit()
        .await
        .context("Failed to commit shuffle transaction")?;

    Ok(())
}

pub async fn get_queue_users(pool: &PgPool, queue_id: i32) -> anyhow::Result<Vec<QueueUserModel>> {
    let users = sqlx::query(
        r#"
        SELECT id, position, priority, is_frozen, queue_id, user_id
        FROM queue_users
        WHERE queue_id = $1
        ORDER BY position
        "#,
    )
    .bind(queue_id)
    .fetch_all(pool)
    .await
    .context("Failed to query queue users")?
    .into_iter()
    .map(|row| QueueUserModel {
        id: row.get("id"),
        position: row.get("position"),
        priority: row.get("priority"),
        is_frozen: row.get("is_frozen"),
        queue_id: row.get("queue_id"),
        user_id: row.get("user_id"),
    })
    .collect();

    Ok(users)
}

pub async fn get_users(
    pool: &PgPool,
    queue_id: i32,
) -> anyhow::Result<Vec<QueueUserWithUserModel>> {
    let users_with_details = sqlx::query(
        r#"
        SELECT
            qu.id,
            qu.position,
            qu.priority,
            qu.is_frozen,
            qu.queue_id,
            qu.user_id,
            u.id as user_id_user,
            u.username,
            u.account_id,
            u.chat_id as chat_id_user,
            u.name
        FROM queue_users qu
        JOIN users u ON qu.user_id = u.id
        WHERE qu.queue_id = $1
        ORDER BY qu.position
        "#,
    )
    .bind(queue_id)
    .fetch_all(pool)
    .await
    .context("Failed to query users with details")?
    .into_iter()
    .map(|row| QueueUserWithUserModel {
        id: row.get("id"),
        position: row.get("position"),
        priority: row.get("priority"),
        is_frozen: row.get("is_frozen"),
        queue_id: row.get("queue_id"),
        user_id: row.get("user_id"),
        user_id_user: row.get("user_id_user"),
        username: row.get("username"),
        account_id: row.get("account_id"),
        chat_id_user: row.get("chat_id_user"),
        name: row.get("name"),
    })
    .collect();

    Ok(users_with_details)
}

pub async fn get_queue_user(
    pool: &PgPool,
    queue_id: i32,
) -> anyhow::Result<Option<QueueUserModel>> {
    let queue_user = sqlx::query(
        r#"
        SELECT id, position, priority, is_frozen, queue_id, user_id
        FROM queue_users
        WHERE queue_id = $1 AND user_id = $2
        "#,
    )
    .bind(queue_id)
    .fetch_optional(pool)
    .await
    .context("Failed to query queue user")?
    .map(|row| QueueUserModel {
        id: row.get("id"),
        position: row.get("position"),
        priority: row.get("priority"),
        is_frozen: row.get("is_frozen"),
        queue_id: row.get("queue_id"),
        user_id: row.get("user_id"),
    });

    Ok(queue_user)
}

pub async fn add_user_to_queue(
    pool: &PgPool,
    queue_id: i32,
    user_id: i32,
    priority: Option<i32>,
) -> anyhow::Result<QueueUserModel> {
    let mut tx = pool
        .begin()
        .await
        .context("Failed to begin transaction for add user to queue")?;

    let max_position_row = sqlx::query(
        r#"
        SELECT MAX(position) as max_pos
        FROM queue_users
        WHERE queue_id = $1
        "#,
    )
    .bind(queue_id)
    .fetch_one(&mut *tx)
    .await
    .context("Failed to get max position for queue")?;

    let next_position = max_position_row
        .get::<Option<i32>, _>("max_pos")
        .unwrap_or(0)
        + 1;

    let new_user = sqlx::query(
        r#"
        INSERT INTO queue_users (queue_id, user_id, priority, position, is_frozen)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, queue_id, user_id, priority, position, is_frozen
        "#,
    )
    .bind(queue_id)
    .bind(user_id)
    .bind(priority)
    .bind(next_position)
    .bind(false)
    .fetch_one(&mut *tx)
    .await
    .context("Failed to insert new queue user")?;

    let new_user = QueueUserModel {
        id: new_user.get("id"),
        queue_id: new_user.get("queue_id"),
        user_id: new_user.get("user_id"),
        priority: new_user.get("priority"),
        position: new_user.get("position"),
        is_frozen: new_user.get("is_frozen"),
    };

    tx.commit()
        .await
        .context("Failed to commit add user to queue transaction")?;

    Ok(new_user)
}

pub async fn remove_user_from_queue(
    pool: &PgPool,
    queue_id: i32,
    user_id: i32,
) -> anyhow::Result<()> {
    let mut tx = pool
        .begin()
        .await
        .context("Failed to begin transaction for removing user")?;

    let queue_user_to_remove = sqlx::query(
        r#"
        SELECT id, position, priority, is_frozen, queue_id, user_id
        FROM queue_users
        WHERE queue_id = $1 AND user_id = $2
        "#,
    )
    .bind(queue_id)
    .bind(user_id)
    .fetch_optional(&mut *tx)
    .await
    .context("Failed to query user to remove")?;

    let queue_user_to_remove = match queue_user_to_remove {
        Some(row) => QueueUserModel {
            id: row.get("id"),
            position: row.get("position"),
            priority: row.get("priority"),
            is_frozen: row.get("is_frozen"),
            queue_id: row.get("queue_id"),
            user_id: row.get("user_id"),
        },
        None => {
            tx.rollback()
                .await
                .context("Failed to rollback remove user transaction (not found)")?;
            return Ok(());
        }
    };

    let user_position = queue_user_to_remove.position;

    let delete_result = sqlx::query(
        r#"
        DELETE FROM queue_users
        WHERE queue_id = $1 AND user_id = $2
        "#,
    )
    .bind(queue_id)
    .bind(user_id)
    .execute(&mut *tx)
    .await
    .context("Failed to delete queue user")?;

    if delete_result.rows_affected() == 0 {
        tx.rollback()
            .await
            .context("Failed to rollback remove user transaction (delete failed)")?;
        bail!("Failed to delete user from queue");
    }

    sqlx::query(
        r#"
        UPDATE queue_users
        SET position = position - 1
        WHERE queue_id = $1 AND position > $2
        "#,
    )
    .bind(queue_id)
    .bind(user_position)
    .execute(&mut *tx)
    .await
    .context("Failed to decrement queue positions")?;

    tx.commit()
        .await
        .context("Failed to commit remove user transaction")?;

    Ok(())
}

pub async fn order_by_priority(pool: &PgPool, queue_id: i32) -> anyhow::Result<()> {
    let mut tx = pool
        .begin()
        .await
        .context("Failed to begin transaction for priority order")?;

    let all_users = sqlx::query(
        r#"
        SELECT id, position, priority, is_frozen, queue_id, user_id
        FROM queue_users
        WHERE queue_id = $1
        ORDER BY priority NULLS LAST, position ASC
        "#,
    )
    .bind(queue_id)
    .fetch_all(&mut *tx)
    .await
    .context("Failed to fetch queue users for priority order")?
    .into_iter()
    .map(|row| QueueUserModel {
        id: row.get("id"),
        position: row.get("position"),
        priority: row.get("priority"),
        is_frozen: row.get("is_frozen"),
        queue_id: row.get("queue_id"),
        user_id: row.get("user_id"),
    })
    .collect::<Vec<_>>();

    if all_users.is_empty() {
        tx.rollback()
            .await
            .context("Failed to rollback empty priority order transaction")?;
        return Ok(());
    }

    for (i, user) in all_users.iter().enumerate() {
        let new_position = i as i32 + 1;
        sqlx::query(
            r#"
            UPDATE queue_users
            SET position = $1
            WHERE id = $2
            "#,
        )
        .bind(new_position)
        .bind(user.id)
        .execute(&mut *tx)
        .await
        .context("Failed to update queue user position")?;
    }

    tx.commit()
        .await
        .context("Failed to commit priority order transaction")?;

    Ok(())
}

pub async fn leave_from_priority_queue(
    pool: &PgPool,
    queue_id: i32,
    user_id: i32,
) -> anyhow::Result<()> {
    let mut tx = pool
        .begin()
        .await
        .context("Failed to begin transaction for leaving priority queue")?;

    let queue_user = sqlx::query(
        r#"
        SELECT id, position, priority, is_frozen, queue_id, user_id
        FROM queue_users
        WHERE queue_id = $1 AND user_id = $2
        "#,
    )
    .bind(queue_id)
    .bind(user_id)
    .fetch_optional(&mut *tx)
    .await
    .context("Failed to query user to leave priority queue")?;

    let queue_user = match queue_user {
        Some(row) => QueueUserModel {
            id: row.get("id"),
            position: row.get("position"),
            priority: row.get("priority"),
            is_frozen: row.get("is_frozen"),
            queue_id: row.get("queue_id"),
            user_id: row.get("user_id"),
        },
        None => {
            tx.rollback().await.context(
                "Failed to rollback leave priority queue transaction (user not in queue)",
            )?;
            return Ok(());
        }
    };

    let original_priority = queue_user.priority;
    let user_position = queue_user.position;

    let delete_result = sqlx::query(
        r#"
        DELETE FROM queue_users
        WHERE id = $1
        "#,
    )
    .bind(queue_user.id)
    .execute(&mut *tx)
    .await
    .context("Failed to delete queue user during leave operation")?;

    if delete_result.rows_affected() == 0 {
        tx.rollback()
            .await
            .context("Failed to rollback leave priority queue transaction (delete failed)")?;
        bail!("Failed to delete user from queue during leave operation");
    }

    sqlx::query(
        r#"
        UPDATE queue_users
        SET position = position - 1
        WHERE queue_id = $1 AND position > $2
        "#,
    )
    .bind(queue_id)
    .bind(user_position)
    .execute(&mut *tx)
    .await
    .context("Failed to decrement queue positions after removing user")?;

    if let Some(_) = original_priority {
        let max_position_row = sqlx::query(
            r#"
            SELECT MAX(position) as max_pos
            FROM queue_users
            WHERE queue_id = $1
            "#,
        )
        .bind(queue_id)
        .fetch_one(&mut *tx)
        .await
        .context("Failed to get max position after deletion for queue")?;

        let next_position = max_position_row
            .get::<Option<i32>, _>("max_pos")
            .unwrap_or(0)
            + 1;

        sqlx::query(
            r#"
            INSERT INTO queue_users (queue_id, user_id, priority, position, is_frozen)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, queue_id, user_id, priority, position, is_frozen
            "#,
        )
        .bind(queue_id)
        .bind(next_position)
        .fetch_one(&mut *tx)
        .await
        .context("Failed to re-insert user into queue with incremented priority")?;
    }

    tx.commit()
        .await
        .context("Failed to commit leave priority queue transaction")?;

    Ok(())
}

pub async fn freeze_user(pool: &PgPool, queue_id: i32, user_id: i32) -> anyhow::Result<()> {
    let result = sqlx::query(
        r#"
        UPDATE queue_users
        SET is_frozen = $1
        WHERE queue_id = $2 AND user_id = $3
        "#,
    )
    .bind(queue_id)
    .bind(user_id)
    .execute(pool)
    .await
    .context("Failed to freeze user in queue")?;

    if result.rows_affected() == 0 {
        bail!("User not found in queue");
    }

    Ok(())
}
