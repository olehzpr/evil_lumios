use std::sync::Arc;

use reqwest::Url;
use teloxide::{
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    Bot,
};

use crate::{
    bot::ui,
    delete_message,
    repositories::timetable_repository::get_entry_by_id,
    state::{Event, State},
};

pub async fn notify(bot: Arc<Bot>, state: State, event: Event) -> anyhow::Result<()> {
    let Event::NotifyTimetable { chat_id, entry_id } = event else {
        return Ok(());
    };
    let entry = get_entry_by_id(&state.db, entry_id).await?;
    let res = ui::timetable_ui::entry_view(entry.clone());

    let bot_username = bot.get_me().await?.user.username.unwrap();

    if let Some(entry) = entry {
        let (inline_text, inline_link) = entry.link.map_or(
            (
                "Додати посилання 🔗",
                format!(
                    "https://t.me/{}?start=edit-timetable_{}",
                    bot_username, entry.id
                ),
            ),
            |link| ("Туда нам нада 🌐", link),
        );

        let new_msg = bot
            .send_message(chat_id, res)
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .reply_markup(InlineKeyboardMarkup::new(vec![vec![
                InlineKeyboardButton::url(inline_text, Url::parse(&inline_link).unwrap()),
            ]]))
            .await?;

        delete_message!(state, new_msg);
    } else {
        let new_msg = bot
            .send_message(chat_id, res)
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .await?;
        delete_message!(state, new_msg);
    }

    Ok(())
}
