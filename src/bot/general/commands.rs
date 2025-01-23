use crate::state::State;
use diesel::{QueryDsl, RunQueryDsl};
use teloxide::{
    payloads::SendMessageSetters, prelude::Requester, types::Message, utils::command::BotCommands,
    Bot,
};

use crate::{
    bot::{timetable::HandlerResult, ui},
    config::{
        commands::Command,
        state::{BotDialogue, StateMachine},
    },
    db::{models::TimetableEntry, StateWithConnection},
    schema,
};

pub async fn help(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;
    Ok(())
}

pub async fn start(bot: Bot, dialogue: BotDialogue, msg: Message, state: State) -> HandlerResult {
    if let Some(text) = msg.text() {
        let param = text.trim_start_matches("/start").trim();
        let id = param.trim_start_matches("edit_timetable_").parse().unwrap();
        let conn = &mut state.conn().await;
        let entry = schema::timetable_entries::table
            .find(id)
            .first::<TimetableEntry>(conn)?;
        bot.send_message(msg.chat.id, ui::timetable::update_link_view(&entry))
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .await?;
        dialogue
            .update(StateMachine::ReceiveEditTimetableEntry { id })
            .await?;

        return Ok(());
    }

    bot.send_message(msg.chat.id, "Hello nigga!").await?;
    Ok(())
}
