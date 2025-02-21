use teloxide::{
    dispatching::dialogue::GetChatId,
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{CallbackQuery, ChatId, MessageId, ReplyParameters},
    Bot,
};

use crate::{
    bot::{handler::HandlerResult, queues::QueueMessages, ui},
    db::{
        self,
        queue::{add_user_to_queue, get_queue_by_id, get_users},
        user::get_user_by_account_id,
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
    let stored_user = get_user_by_account_id(&state, query.from.id).await?;
    if let Err(err) = add_user_to_queue(&state.db, queue_id, stored_user.id, None).await {
        tracing::error!("Failed to join queue: {:?}", err);
        bot.answer_callback_query(query.id).await?;
        return Ok(());
    };
    let queue = get_queue_by_id(&state.db, queue_id).await?;
    let users = get_users(&state.db, queue_id).await?;

    tracing::debug!("{:?}", queue);

    let message_id = queue.message_id.parse::<i32>().unwrap();
    let chat_id = queue.chat_id.parse::<i64>().unwrap();
    let updated_content = ui::queue::regular_queue(&queue, users);

    tracing::debug!("{:?}", updated_content);

    if queue.is_mixed.is_some() {
        tracing::debug!("Editing mixed queue");
        bot.edit_mixed_queue(
            ChatId(chat_id),
            MessageId(message_id),
            queue.id,
            &updated_content,
        )
        .await;
    } else if queue.is_priority {
        tracing::debug!("Editing priority queue");
        bot.edit_priority_queue(
            ChatId(chat_id),
            MessageId(message_id),
            queue.id,
            &updated_content,
        )
        .await;
    } else {
        tracing::debug!("Editing regular queue");
        bot.edit_regular_queue(
            ChatId(chat_id),
            MessageId(message_id),
            queue.id,
            &updated_content,
        )
        .await;
    }

    Ok(())
}

pub async fn leave_queue(
    bot: Bot,
    state: State,
    queue_id: i32,
    query: CallbackQuery,
) -> HandlerResult {
    tracing::debug!("Leaving queue {}", queue_id);
    let stored_user = get_user_by_account_id(&state, query.from.id).await?;
    db::queue::remove_user_from_queue(&state.db, queue_id, stored_user.id).await?;

    let queue = get_queue_by_id(&state.db, queue_id).await?;
    let users = get_users(&state.db, queue_id).await?;

    let message_id = queue.message_id.parse::<i32>().unwrap();
    let chat_id = queue.chat_id.parse::<i64>().unwrap();
    let updated_content = ui::queue::regular_queue(&queue, users);

    tracing::debug!("{:?}", updated_content);

    if queue.is_mixed.is_some() {
        bot.edit_mixed_queue(
            ChatId(chat_id),
            MessageId(message_id),
            queue.id,
            &updated_content,
        )
        .await;
    } else if queue.is_priority {
        bot.edit_priority_queue(
            ChatId(chat_id),
            MessageId(message_id),
            queue.id,
            &updated_content,
        )
        .await;
    } else {
        bot.edit_regular_queue(
            ChatId(chat_id),
            MessageId(message_id),
            queue.id,
            &updated_content,
        )
        .await;
    }
    Ok(())
}

pub async fn delete_queue(
    bot: Bot,
    state: State,
    queue_id: i32,
    query: CallbackQuery,
) -> HandlerResult {
    tracing::debug!("Deleting queue {}", queue_id);
    let queue = get_queue_by_id(&state.db, queue_id).await?;

    db::queue::delete_queue(&state.db, queue.id).await?;

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
    tracing::debug!("Notifying queue {}", queue_id);
    let queue = get_queue_by_id(&state.db, queue_id).await?;
    let users = get_users(&state.db, queue_id).await?;
    if users.is_empty() {
        return Ok(());
    }
    let queue_message_id = queue.message_id.parse::<i32>().unwrap();
    bot.send_message(
        query.chat_id().unwrap(),
        ui::queue::notification(&users[0], &queue),
    )
    .reply_parameters(ReplyParameters::new(MessageId(queue_message_id)))
    .await?;
    Ok(())
}

pub async fn shuffle_queue(
    bot: Bot,
    state: State,
    queue_id: i32,
    query: CallbackQuery,
) -> HandlerResult {
    tracing::debug!("Shuffling queue {}", queue_id);
    db::queue::shuffle_queue(&state.db, queue_id).await?;

    let queue = get_queue_by_id(&state.db, queue_id).await?;
    let users = get_users(&state.db, queue_id).await?;

    let message_id = queue.message_id.parse::<i32>().unwrap();
    let updated_content = ui::queue::regular_queue(&queue, users);

    bot.edit_mixed_queue(
        query.chat_id().unwrap(),
        MessageId(message_id),
        queue.id,
        &updated_content,
    )
    .await;

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

    db::queue::freeze_user(&state.db, queue_id, user_who_clicked.id).await?;

    let queue = get_queue_by_id(&state.db, queue_id).await?;
    let users = get_users(&state.db, queue_id).await?;

    let message_id = queue.message_id.parse::<i32>().unwrap();
    let updated_content = ui::queue::regular_queue(&queue, users);

    bot.edit_priority_queue(
        query.chat_id().unwrap(),
        MessageId(message_id),
        queue.id,
        &updated_content,
    )
    .await;

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

    db::queue::leave_from_priority_queue(&state.db, queue_id, user_who_clicked.id).await?;

    let queue = get_queue_by_id(&state.db, queue_id).await?;
    let users = get_users(&state.db, queue_id).await?;

    let message_id = queue.message_id.parse::<i32>().unwrap();
    let updated_content = ui::queue::regular_queue(&queue, users);

    bot.edit_priority_queue(
        query.chat_id().unwrap(),
        MessageId(message_id),
        queue.id,
        &updated_content,
    )
    .await;

    Ok(())
}
