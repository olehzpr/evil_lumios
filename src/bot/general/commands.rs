use crate::{
    bot::{
        externsions::{ExtendedBot, Msg},
        handler::HandlerResult,
    },
    state::State,
};
use diesel::{QueryDsl, RunQueryDsl};
use reqwest::Url;
use teloxide::{
    payloads::{SendMessageSetters, SendPhotoSetters},
    prelude::{Request, Requester},
    types::{InlineKeyboardButton, InlineKeyboardMarkup, InputFile, Message, WebAppInfo},
    utils::command::BotCommands,
    Bot,
};

use crate::{
    bot::ui,
    config::{
        commands::Command,
        state::{BotDialogue, StateMachine},
    },
    db::{models::TimetableEntry, StateWithConnection},
    schema,
};

use super::StartCommand;

pub async fn help(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;
    Ok(())
}

pub async fn start(bot: Bot, dialogue: BotDialogue, msg: Message, state: State) -> HandlerResult {
    if let Some(text) = msg.text() {
        let command = StartCommand::from_str(text);
        match command {
            Some(StartCommand::Start) => {
                default(bot, msg).await?;
            }
            Some(StartCommand::EditTimetable { entry_id }) => {
                edit_timetable_entry(entry_id, bot, dialogue, msg, state).await?;
            }
            Some(StartCommand::Casino) => {
                enter_casino(bot, msg).await?;
            }
            None => {
                default(bot, msg).await?;
            }
        }

        return Ok(());
    }

    default(bot, msg).await?;
    Ok(())
}

async fn default(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Hello nigga!").await?;
    Ok(())
}

async fn edit_timetable_entry(
    entry_id: i32,
    bot: Bot,
    dialogue: BotDialogue,
    msg: Message,
    state: State,
) -> anyhow::Result<()> {
    let conn = &mut state.conn().await;
    let entry = schema::timetable_entries::table
        .find(entry_id)
        .first::<TimetableEntry>(conn)?;
    bot.send_message(msg.chat.id, ui::timetable::update_link_view(&entry))
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .await?;
    dialogue
        .update(StateMachine::ReceiveEditTimetableEntry { id: entry_id })
        .await?;

    Ok(())
}

async fn enter_casino(bot: Bot, msg: Message) -> HandlerResult {
    let (res, img_url) = ui::stats::casino_arrival();
    bot.send_photo(msg.chat.id, InputFile::url(Url::parse(&img_url).unwrap()))
        .caption(&res)
        .reply_markup(InlineKeyboardMarkup::new(vec![vec![
            InlineKeyboardButton::web_app(
                "Зайти в казино",
                WebAppInfo {
                    url: Url::parse("https://evil-lumios-web.vercel.app/").unwrap(),
                },
            ),
        ]]))
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .send()
        .await?;

    Ok(())
}
