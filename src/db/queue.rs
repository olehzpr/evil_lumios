use diesel::{
    BoolExpressionMethods, ExpressionMethods, OptionalExtension, PgConnection, QueryDsl,
    RunQueryDsl,
};
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use teloxide::types::{ChatId, MessageId};

use crate::schema::{queue_users, queues, users};

use super::models::{NewQueue, NewQueueUser, Queue, QueueUser, User};

pub fn create_queue(conn: &mut PgConnection, new_queue: NewQueue) -> anyhow::Result<Queue> {
    let new_queue = diesel::insert_into(queues::table)
        .values(new_queue)
        .get_result::<Queue>(conn)?;

    Ok(new_queue)
}

pub fn get_all_queues(conn: &mut PgConnection, chat_id: ChatId) -> anyhow::Result<Vec<Queue>> {
    let queues = queues::table
        .filter(queues::chat_id.eq(chat_id.to_string()))
        .filter(queues::is_deleted.eq(false))
        .select(queues::all_columns)
        .load::<Queue>(conn)?;

    Ok(queues)
}

pub fn delete_queue(conn: &mut PgConnection, queue_id: i32) -> anyhow::Result<()> {
    diesel::update(queues::table)
        .filter(queues::id.eq(queue_id))
        .set(queues::is_deleted.eq(true))
        .execute(conn)?;

    Ok(())
}

pub fn get_queue(
    conn: &mut PgConnection,
    chat_id: ChatId,
    message_id: MessageId,
) -> anyhow::Result<Queue> {
    let queue = queues::table
        .filter(queues::chat_id.eq(chat_id.to_string()))
        .filter(queues::message_id.eq(message_id.to_string()))
        .filter(queues::is_deleted.eq(false))
        .select(queues::all_columns)
        .first::<Queue>(conn)?;

    Ok(queue)
}

pub fn get_queue_by_id(conn: &mut PgConnection, queue_id: i32) -> anyhow::Result<Queue> {
    let queue = queues::table
        .filter(queues::id.eq(queue_id))
        .filter(queues::is_deleted.eq(false))
        .select(queues::all_columns)
        .first::<Queue>(conn)?;

    Ok(queue)
}

pub fn shuffle_queue(conn: &mut PgConnection, queue_id: i32) -> anyhow::Result<()> {
    let mut rng = StdRng::from_entropy();
    let mut all_users = get_queue_users(conn, queue_id)?;

    all_users.shuffle(&mut rng);

    for (i, user) in all_users.iter().enumerate() {
        let new_position = i as i32 + 1;
        diesel::update(queue_users::table)
            .filter(
                queue_users::queue_id
                    .eq(queue_id)
                    .and(queue_users::user_id.eq(user.id)),
            )
            .set(queue_users::position.eq(new_position))
            .execute(conn)?;
    }

    Ok(())
}

pub fn get_queue_users(conn: &mut PgConnection, queue_id: i32) -> anyhow::Result<Vec<QueueUser>> {
    let users = queue_users::table
        .filter(queue_users::queue_id.eq(queue_id))
        .select(queue_users::all_columns)
        .order(queue_users::position.asc())
        .load::<QueueUser>(conn)?;

    Ok(users)
}

pub fn get_users(conn: &mut PgConnection, queue_id: i32) -> anyhow::Result<Vec<User>> {
    let users = queue_users::table
        .filter(queue_users::queue_id.eq(queue_id))
        .inner_join(users::table)
        .select(users::all_columns)
        .order(queue_users::position.asc())
        .load::<User>(conn)?;

    Ok(users)
}

pub fn get_queue_user(
    conn: &mut PgConnection,
    queue_id: i32,
    user_id: i32,
) -> anyhow::Result<Option<QueueUser>> {
    let queue_user = queue_users::table
        .filter(
            queue_users::queue_id
                .eq(queue_id)
                .and(queue_users::user_id.eq(user_id)),
        )
        .select(queue_users::all_columns)
        .first::<QueueUser>(conn)
        .optional()?;

    Ok(queue_user)
}

pub fn add_user_to_queue(
    conn: &mut PgConnection,
    queue_id: i32,
    user_id: i32,
    priority: Option<i32>,
) -> anyhow::Result<QueueUser> {
    let num_of_users = queue_users::table
        .filter(queue_users::queue_id.eq(queue_id))
        .count()
        .get_result::<i64>(conn)?;
    let next_position = num_of_users + 1;
    let new_user = diesel::insert_into(queue_users::table)
        .values(NewQueueUser {
            queue_id,
            user_id,
            priority,
            position: next_position as i32,
        })
        .get_result::<QueueUser>(conn)?;

    Ok(new_user)
}

pub fn remove_user_from_queue(
    conn: &mut PgConnection,
    queue_id: i32,
    user_id: i32,
) -> anyhow::Result<()> {
    let queue_user = get_queue_user(conn, queue_id, user_id)?;

    if queue_user.is_none() {
        return Ok(());
    }
    let user_pos = queue_user.unwrap().position;

    let all_users = get_queue_users(conn, queue_id)?;
    let mut shift = 1;

    for user in all_users.iter() {
        if user.position < user_pos {
            continue;
        }
        if user.position == user_pos {
            diesel::delete(queue_users::table)
                .filter(
                    queue_users::queue_id
                        .eq(queue_id)
                        .and(queue_users::user_id.eq(user.id)),
                )
                .execute(conn)?;
            continue;
        }
        if user.is_freezed.unwrap_or(false) {
            shift += 1;
            continue;
        }
        diesel::update(queue_users::table)
            .filter(
                queue_users::queue_id
                    .eq(queue_id)
                    .and(queue_users::user_id.eq(user.id)),
            )
            .set(queue_users::position.eq(user.position - shift))
            .execute(conn)?;
        shift = 1;
    }

    Ok(())
}

pub fn order_by_priority(conn: &mut PgConnection, queue_id: i32) -> anyhow::Result<()> {
    let all_users = queue_users::table
        .filter(queue_users::queue_id.eq(queue_id))
        .select(queue_users::all_columns)
        .order(queue_users::position.asc())
        .order(queue_users::priority.asc())
        .load::<QueueUser>(conn)?;
    for (i, user) in all_users.iter().enumerate() {
        let new_position = i as i32 + 1;
        diesel::update(queue_users::table)
            .filter(
                queue_users::queue_id
                    .eq(queue_id)
                    .and(queue_users::user_id.eq(user.id)),
            )
            .set(queue_users::position.eq(new_position))
            .execute(conn)?;
    }
    Ok(())
}

pub fn leave_from_priority_queue(
    conn: &mut PgConnection,
    queue_id: i32,
    user_id: i32,
) -> anyhow::Result<()> {
    let queue_user = get_queue_user(conn, queue_id, user_id)?;
    if let Some(user) = queue_user {
        remove_user_from_queue(conn, queue_id, user_id)?;
        if let Some(priority) = user.priority {
            add_user_to_queue(conn, queue_id, user_id, Some(priority + 1))?;
        }
    }
    Ok(())
}

pub fn freeze_user(conn: &mut PgConnection, queue_id: i32, user_id: i32) -> anyhow::Result<()> {
    let queue_user = get_queue_user(conn, queue_id, user_id)?;
    if let Some(user) = queue_user {
        diesel::update(queue_users::table)
            .filter(
                queue_users::queue_id
                    .eq(queue_id)
                    .and(queue_users::user_id.eq(user.id)),
            )
            .set(queue_users::is_freezed.eq(true))
            .execute(conn)?;
    }
    Ok(())
}
