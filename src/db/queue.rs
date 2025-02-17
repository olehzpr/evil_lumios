use rand::seq::SliceRandom;
use rand::SeedableRng;
use sea_orm::{entity::*, query::*, DatabaseConnection};
use teloxide::types::{ChatId, MessageId};

use crate::entities::{queue_users, queues, users};

use crate::entities::queue_users::Entity as QueueUser;
use crate::entities::queues::Entity as Queue;
use crate::entities::users::Entity as User;

pub async fn create_queue(
    conn: &DatabaseConnection,
    new_queue: queues::ActiveModel,
) -> anyhow::Result<queues::Model> {
    let new_queue = new_queue.insert(conn).await?;

    Ok(new_queue)
}

pub async fn get_all_queues(
    conn: &DatabaseConnection,
    chat_id: ChatId,
) -> anyhow::Result<Vec<queues::Model>> {
    let queues = Queue::find()
        .filter(queues::Column::ChatId.eq(chat_id.to_string()))
        .filter(queues::Column::IsDeleted.eq(false))
        .all(conn)
        .await?;

    Ok(queues)
}

pub async fn delete_queue(conn: &DatabaseConnection, queue_id: i32) -> anyhow::Result<()> {
    let queue: Option<queues::ActiveModel> =
        Queue::find_by_id(queue_id).one(conn).await?.map(Into::into);

    if let Some(mut queue) = queue {
        queue.is_deleted = Set(true);
        queue.update(conn).await?;
    }

    Ok(())
}

pub async fn get_queue(
    conn: &DatabaseConnection,
    chat_id: ChatId,
    message_id: MessageId,
) -> anyhow::Result<queues::Model> {
    let queue = Queue::find()
        .filter(queues::Column::ChatId.eq(chat_id.to_string()))
        .filter(queues::Column::MessageId.eq(message_id.to_string()))
        .filter(queues::Column::IsDeleted.eq(false))
        .one(conn)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Queue not found"))?;

    Ok(queue)
}

pub async fn get_queue_by_id(
    conn: &DatabaseConnection,
    queue_id: i32,
) -> anyhow::Result<queues::Model> {
    let queue = Queue::find()
        .filter(queues::Column::Id.eq(queue_id))
        .filter(queues::Column::IsDeleted.eq(false))
        .one(conn)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Queue not found"))?;

    Ok(queue)
}

pub async fn shuffle_queue(conn: &DatabaseConnection, queue_id: i32) -> anyhow::Result<()> {
    let mut rng = rand::rngs::StdRng::from_entropy();
    let mut all_users = get_queue_users(conn, queue_id).await?;

    all_users.shuffle(&mut rng);

    for (i, user) in all_users.iter().enumerate() {
        let new_position = i as i32 + 1;
        let mut user: queue_users::ActiveModel = user.clone().into();
        user.position = Set(new_position);
        user.update(conn).await?;
    }

    Ok(())
}

pub async fn get_queue_users(
    conn: &DatabaseConnection,
    queue_id: i32,
) -> anyhow::Result<Vec<queue_users::Model>> {
    let users = QueueUser::find()
        .filter(queue_users::Column::QueueId.eq(queue_id))
        .order_by_asc(queue_users::Column::Position)
        .all(conn)
        .await?;

    Ok(users)
}

pub async fn get_users(
    conn: &DatabaseConnection,
    queue_id: i32,
) -> anyhow::Result<Vec<users::Model>> {
    let users = QueueUser::find()
        .filter(queue_users::Column::QueueId.eq(queue_id))
        .find_also_related(User)
        .order_by_asc(queue_users::Column::Position)
        .all(conn)
        .await?
        .into_iter()
        .filter_map(|(_, user)| user)
        .collect();

    Ok(users)
}

pub async fn get_queue_user(
    conn: &DatabaseConnection,
    queue_id: i32,
    user_id: i32,
) -> anyhow::Result<Option<queue_users::Model>> {
    let queue_user = QueueUser::find()
        .filter(
            queue_users::Column::QueueId
                .eq(queue_id)
                .and(queue_users::Column::UserId.eq(user_id)),
        )
        .one(conn)
        .await?;

    Ok(queue_user)
}

pub async fn add_user_to_queue(
    conn: &DatabaseConnection,
    queue_id: i32,
    user_id: i32,
    priority: Option<i32>,
) -> anyhow::Result<queue_users::Model> {
    let num_of_users = QueueUser::find()
        .filter(queue_users::Column::QueueId.eq(queue_id))
        .count(conn)
        .await?;
    let next_position = num_of_users + 1;
    let new_user = queue_users::ActiveModel {
        queue_id: Set(queue_id),
        user_id: Set(user_id),
        priority: Set(priority),
        position: Set(next_position as i32),
        ..Default::default()
    }
    .insert(conn)
    .await?;

    Ok(new_user)
}

pub async fn remove_user_from_queue(
    conn: &DatabaseConnection,
    queue_id: i32,
    user_id: i32,
) -> anyhow::Result<()> {
    let queue_user = get_queue_user(conn, queue_id, user_id).await?;

    if queue_user.is_none() {
        return Ok(());
    }
    let user_pos = queue_user.unwrap().position;

    let all_users = get_queue_users(conn, queue_id).await?;
    let mut shift = 1;

    for user in all_users.iter() {
        if user.position < user_pos {
            continue;
        }
        if user.position == user_pos {
            QueueUser::delete_many()
                .filter(
                    queue_users::Column::QueueId
                        .eq(queue_id)
                        .and(queue_users::Column::UserId.eq(user_id)),
                )
                .exec(conn)
                .await?;
            continue;
        }
        if user.is_freezed.unwrap_or(false) {
            shift += 1;
            continue;
        }
        let mut user: queue_users::ActiveModel = user.clone().into();
        user.position = Set(user.position.unwrap() - shift);
        user.update(conn).await?;
        shift = 1;
    }

    Ok(())
}

pub async fn order_by_priority(conn: &DatabaseConnection, queue_id: i32) -> anyhow::Result<()> {
    let all_users = QueueUser::find()
        .filter(queue_users::Column::QueueId.eq(queue_id))
        .order_by_asc(queue_users::Column::Position)
        .order_by_asc(queue_users::Column::Priority)
        .all(conn)
        .await?;

    for (i, user) in all_users.iter().enumerate() {
        let new_position = i as i32 + 1;
        let mut user: queue_users::ActiveModel = user.clone().into();
        user.position = Set(new_position);
        user.update(conn).await?;
    }

    Ok(())
}

pub async fn leave_from_priority_queue(
    conn: &DatabaseConnection,
    queue_id: i32,
    user_id: i32,
) -> anyhow::Result<()> {
    let queue_user = get_queue_user(conn, queue_id, user_id).await?;
    if let Some(user) = queue_user {
        remove_user_from_queue(conn, queue_id, user_id).await?;
        if let Some(priority) = user.priority {
            add_user_to_queue(conn, queue_id, user_id, Some(priority + 1)).await?;
        }
    }
    Ok(())
}

pub async fn freeze_user(
    conn: &DatabaseConnection,
    queue_id: i32,
    user_id: i32,
) -> anyhow::Result<()> {
    let queue_user = get_queue_user(conn, queue_id, user_id).await?;
    if let Some(user) = queue_user {
        let mut user: queue_users::ActiveModel = user.into();
        user.is_freezed = Set(Some(true));
        user.update(conn).await?;
    }
    Ok(())
}
