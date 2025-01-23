use evil_lumios::State;
use teloxide::{prelude::Requester, types::MessageReactionUpdated, Bot};

use crate::bot::timetable::HandlerResult;

pub async fn handle_reaction(
    bot: Bot,
    message_reaction: MessageReactionUpdated,
    _state: State,
) -> HandlerResult {
    bot.send_message(message_reaction.chat.id, "Reaction detected")
        .await?;
    Ok(())
}
