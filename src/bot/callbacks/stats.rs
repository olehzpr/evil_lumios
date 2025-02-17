use teloxide::{
    payloads::EditMessageTextSetters,
    prelude::Requester,
    types::{CallbackQuery, MessageId, UserId},
    Bot,
};

use crate::{bot::handler::HandlerResult, db::stats::get_full_me, state::State};

pub async fn show_full_stats(
    bot: Bot,
    state: State,
    message_id: MessageId,
    user_id: UserId,
    query: CallbackQuery,
) -> HandlerResult {
    tracing::debug!("message_id: {:?}, user_id: {:?}", message_id, user_id);
    let stats = get_full_me(&state.db, user_id).await?;
    let res = crate::bot::ui::stats::full_stats(stats);
    let chat_id = query.message.as_ref().unwrap().chat().id;
    bot.edit_message_text(chat_id, message_id, res)
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .await?;
    bot.answer_callback_query(query.id).await?;
    Ok(())
}
