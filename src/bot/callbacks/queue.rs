use teloxide::{
    dispatching::dialogue::GetChatId,
    prelude::Requester,
    types::{CallbackQuery, ChatId, MessageId},
    Bot,
};

use crate::{
    bot::{handler::HandlerResult, queues::QueueMessages, ui},
    db::{
        self,
        queue::{add_user_to_queue, get_queue_by_id, get_users},
        user::get_user_by_account_id,
        StateWithConnection,
    },
    state::State,
};

pub async fn join_queue(
    bot: Bot,
    state: State,
    queue_id: i32,
    query: CallbackQuery,
) -> HandlerResult {
    tracing::debug!("Joining queue {}", queue_id);
    let conn = &mut state.conn().await;
    let stored_user = get_user_by_account_id(&state, query.from.id).await?;
    if let Err(_) = add_user_to_queue(conn, queue_id, stored_user.id, None) {
        bot.answer_callback_query(query.id).await?;
        return Ok(());
    };
    let queue = get_queue_by_id(conn, queue_id)?;
    let users = get_users(conn, queue_id)?;

    let message_id = queue.message_id.parse::<i32>().unwrap();
    let chat_id = queue.chat_id.parse::<i64>().unwrap();
    let updated_content = ui::queue::regular_queue(&queue, users);

    bot.edit_regular_queue(
        ChatId(chat_id),
        MessageId(message_id),
        queue.id,
        &updated_content,
    )
    .await;

    Ok(())
}

pub async fn leave_queue(
    bot: Bot,
    state: State,
    queue_id: i32,
    query: CallbackQuery,
) -> HandlerResult {
    Ok(())
}

pub async fn delete_queue(
    bot: Bot,
    state: State,
    queue_id: i32,
    query: CallbackQuery,
) -> HandlerResult {
    tracing::debug!("Deleting queue {}", queue_id);
    let conn = &mut state.conn().await;
    let queue = get_queue_by_id(conn, queue_id)?;

    db::queue::delete_queue(conn, queue.id)?;

    let message_id = queue.message_id.clone().parse::<i32>().unwrap();

    bot.delete_message(query.chat_id().unwrap(), MessageId(message_id))
        .await?;

    Ok(())
}

pub async fn notify_queue(
    bot: Bot,
    state: State,
    queue_id: i32,
    query: CallbackQuery,
) -> HandlerResult {
    Ok(())
}

pub async fn shuffle_queue(
    bot: Bot,
    state: State,
    queue_id: i32,
    query: CallbackQuery,
) -> HandlerResult {
    Ok(())
}

pub async fn freeze_queue(
    bot: Bot,
    state: State,
    queue_id: i32,
    query: CallbackQuery,
) -> HandlerResult {
    Ok(())
}

pub async fn skip_queue(
    bot: Bot,
    state: State,
    queue_id: i32,
    query: CallbackQuery,
) -> HandlerResult {
    Ok(())
}
