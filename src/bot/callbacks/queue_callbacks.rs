use teloxide::{
    dispatching::dialogue::GetChatId,
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{CallbackQuery, ChatId, MessageId, ReplyParameters},
    Bot,
};

use crate::{
    bot::{handler::HandlerResult, queues::QueueMessages, ui},
    delete_message,
    repositories::{
        self,
        queue_repository::{add_user_to_queue, get_queue_by_id, get_users},
        user_repository::get_user_by_account_id,
    },
    state::State,
};

pub async fn join_queue(
    bot: Bot,
    state: State,
    queue_id: i32,
    query: CallbackQuery,
) -> HandlerResult {
    let stored_user = get_user_by_account_id(&state, query.from.id).await?;

    if let Err(err) = add_user_to_queue(&state.db, queue_id, stored_user.id, None).await {
        tracing::error!("Failed to join queue: {:?}", err);
        bot.answer_callback_query(query.id).await?;
        return Ok(());
    };

    let queue = get_queue_by_id(&state.db, queue_id).await?;
    let users = get_users(&state.db, queue_id).await?;

    tracing::debug!("{:?}", queue);

    bot.edit_queue(queue, users).await;

    Ok(())
}

pub async fn leave_queue(
    bot: Bot,
    state: State,
    queue_id: i32,
    query: CallbackQuery,
) -> HandlerResult {
    let stored_user = get_user_by_account_id(&state, query.from.id).await?;
    repositories::queue_repository::remove_user_from_queue(&state.db, queue_id, stored_user.id)
        .await?;

    let queue = get_queue_by_id(&state.db, queue_id).await?;
    let users = get_users(&state.db, queue_id).await?;

    bot.edit_queue(queue, users).await;

    Ok(())
}

pub async fn delete_queue(
    bot: Bot,
    state: State,
    queue_id: i32,
    query: CallbackQuery,
) -> HandlerResult {
    let chat_member = bot
        .get_chat_member(query.chat_id().unwrap(), query.from.id)
        .await?;
    if !chat_member.is_privileged() {
        bot.answer_callback_query(query.id).await?;
        return Ok(());
    }

    let queue = get_queue_by_id(&state.db, queue_id).await?;

    repositories::queue_repository::delete_queue(&state.db, queue.id).await?;

    bot.delete_message(ChatId(queue.chat_id), MessageId(queue.message_id))
        .await?;

    Ok(())
}

pub async fn notify_queue(
    bot: Bot,
    state: State,
    queue_id: i32,
    _query: CallbackQuery,
) -> HandlerResult {
    tracing::debug!("Notifying queue {}", queue_id);
    let queue = get_queue_by_id(&state.db, queue_id).await?;
    let users = get_users(&state.db, queue_id).await?;
    if users.is_empty() {
        return Ok(());
    }
    let new_msg = bot
        .send_message(
            ChatId(queue.chat_id),
            ui::queue_ui::notification(&users[0], &queue),
        )
        .reply_parameters(ReplyParameters::new(MessageId(queue.message_id)))
        .await?;

    delete_message!(state, new_msg);

    Ok(())
}

pub async fn shuffle_queue(
    bot: Bot,
    state: State,
    queue_id: i32,
    query: CallbackQuery,
) -> HandlerResult {
    let chat_member = bot
        .get_chat_member(query.chat_id().unwrap(), query.from.id)
        .await?;
    if !chat_member.is_privileged() {
        bot.answer_callback_query(query.id).await?;
        return Ok(());
    }

    tracing::debug!("Shuffling queue with id: {}", queue_id);
    repositories::queue_repository::shuffle_queue(&state.db, queue_id).await?;

    let queue = get_queue_by_id(&state.db, queue_id).await?;
    let users = get_users(&state.db, queue_id).await?;

    bot.edit_queue(queue, users).await;

    Ok(())
}

pub async fn freeze_queue(
    bot: Bot,
    state: State,
    queue_id: i32,
    query: CallbackQuery,
) -> HandlerResult {
    tracing::debug!("Freezing queue {}", queue_id);

    let user_id = query.from.id;
    let user_who_clicked = get_user_by_account_id(&state, user_id).await?;

    repositories::queue_repository::freeze_user(&state.db, queue_id, user_who_clicked.id).await?;

    let queue = get_queue_by_id(&state.db, queue_id).await?;
    let users = get_users(&state.db, queue_id).await?;

    bot.edit_queue(queue, users).await;

    Ok(())
}

pub async fn skip_queue(
    bot: Bot,
    state: State,
    queue_id: i32,
    query: CallbackQuery,
) -> HandlerResult {
    tracing::debug!("Skipping queue {}", queue_id);

    let user_id = query.from.id;
    let user_who_clicked = get_user_by_account_id(&state, user_id).await?;

    repositories::queue_repository::skip_priority_queue(
        &state.db,
        queue_id,
        user_who_clicked.id,
        false,
    )
    .await?;

    let queue = get_queue_by_id(&state.db, queue_id).await?;
    let users = get_users(&state.db, queue_id).await?;

    bot.edit_queue(queue, users).await;

    Ok(())
}

pub async fn done_queue(
    bot: Bot,
    state: State,
    queue_id: i32,
    query: CallbackQuery,
) -> HandlerResult {
    tracing::debug!("Skipping queue {}", queue_id);

    let user_id = query.from.id;
    let user_who_clicked = get_user_by_account_id(&state, user_id).await?;

    repositories::queue_repository::skip_priority_queue(
        &state.db,
        queue_id,
        user_who_clicked.id,
        true,
    )
    .await?;

    let queue = get_queue_by_id(&state.db, queue_id).await?;
    let users = get_users(&state.db, queue_id).await?;

    bot.edit_queue(queue, users).await;

    Ok(())
}
