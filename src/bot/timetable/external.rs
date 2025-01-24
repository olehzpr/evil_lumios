use crate::{bot::handler::HandlerResult, state::State};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use teloxide::{prelude::Requester, types::Message, Bot};

use crate::{
    config::state::{BotDialogue, StateMachine},
    db::StateWithConnection,
    schema,
};

pub async fn receive_timetable_entry_link(
    bot: Bot,
    dialogue: BotDialogue,
    msg: Message,
    id: i32,
    state: State,
) -> HandlerResult {
    match msg.text() {
        Some(link) => {
            if !link.starts_with("http") {
                bot.send_message(
                    msg.chat.id,
                    "Неправильне посилання, надішліть посилання що починається з http:// або https://",
                )
                .await?;
                dialogue
                    .update(StateMachine::ReceiveEditTimetableEntry { id })
                    .await?;
                return Ok(());
            }
            let conn = &mut state.conn().await;
            diesel::update(schema::timetable_entries::table.find(id))
                .set(schema::timetable_entries::link.eq(link))
                .execute(conn)?;
            bot.send_message(msg.chat.id, "Посилання успішно змінено")
                .await?;
            dialogue.exit().await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Надішліть посилання повторно")
                .await?;
        }
    }
    Ok(())
}
