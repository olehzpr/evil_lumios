use crate::{bot::handler::HandlerResult, state::State};
use teloxide::{prelude::Requester, types::MessageReactionUpdated, Bot};

pub async fn handle_reaction(
    bot: Bot,
    message_reaction: MessageReactionUpdated,
    _state: State,
) -> HandlerResult {
    bot.send_message(message_reaction.chat.id, "Reaction detected")
        .await?;
    Ok(())
}
