use diesel::{QueryDsl, RunQueryDsl};
use evil_lumios::State;
use teloxide::{
    payloads::SendMessageSetters, prelude::Requester, types::Message, utils::command::BotCommands,
    Bot,
};

use crate::{
    bot::{timetable::HandlerResult, ui::change_timetable_entry_request_view},
    config::{
        commands::Command,
        state::{BotDialogue, StateMachine},
    },
    db::{connection, models::TimetableEntry},
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
        let conn = &mut connection(&state).await;
        let entry = schema::timetable_entries::table
            .find(id)
            .first::<TimetableEntry>(conn)?;
        bot.send_message(msg.chat.id, change_timetable_entry_request_view(&entry))
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
