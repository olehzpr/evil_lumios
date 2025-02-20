use crate::{bot::handler::HandlerResult, delete_message, param, redis::RedisCache};
use reqwest::Url;
use teloxide::{
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, LinkPreviewOptions, Message},
    Bot,
};

use crate::{
    bot::ui::{self},
    db::timetable::{
        get_current_entry, get_full_timetable, get_next_entry, get_today_timetable,
        get_tomorrow_timetable, get_week_timetable, import_timetable,
    },
    State,
};

pub async fn import(bot: Bot, msg: Message, state: State) -> HandlerResult {
    let group_name = param!(
        bot,
        msg,
        state,
        String,
        "Вкажіть назву групи, наприклад ІП-32"
    );

    let group_api_result = state
        .http_client
        .get("https://api.campus.kpi.ua/group/find")
        .query(&[("name", &group_name)])
        .send()
        .await?;
    let group_json = group_api_result.json::<serde_json::Value>().await?;
    let group_id = group_json
        .as_array()
        .and_then(|array| array.iter().find(|x| x["name"] == group_name))
        .and_then(|group| group.get("id"))
        .and_then(|id| id.as_str())
        .ok_or_else(|| anyhow::anyhow!("Виникла помилка під час імпортування розкладу"))?;
    let timetable_api_result = state
        .http_client
        .get("https://api.campus.kpi.ua/schedule/lessons")
        .query(&[
            ("groupId", group_id.to_string()),
            ("groupName", group_name.to_string()),
        ])
        .send()
        .await?;
    let timetable = timetable_api_result.json::<serde_json::Value>().await?;

    import_timetable(&state.db, &msg.chat.id.to_string(), timetable).await?;
    state.redis.clear_timetable_entries(msg.chat.id)?;

    let new_msg = bot
        .send_message(msg.chat.id, "Розклад успішно імпортовано ✅")
        .await?;

    delete_message!(state, msg);
    delete_message!(state, new_msg);

    Ok(())
}

pub async fn today(bot: Bot, msg: Message, state: State) -> HandlerResult {
    let entries = get_today_timetable(&state.db, &msg.chat.id.to_string()).await?;
    let res = ui::timetable::day_view(entries);

    let new_msg = bot
        .send_message(msg.chat.id, res)
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .link_preview_options(DISABLED_LINK_PREVIEW_OPTIONS)
        .await?;

    delete_message!(state, msg);
    delete_message!(state, new_msg);

    Ok(())
}

pub async fn tomorrow(bot: Bot, msg: Message, state: State) -> HandlerResult {
    let entries = get_tomorrow_timetable(&state.db, &msg.chat.id.to_string()).await?;
    let res = ui::timetable::day_view(entries);

    let new_msg = bot
        .send_message(msg.chat.id, res)
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .link_preview_options(DISABLED_LINK_PREVIEW_OPTIONS)
        .await?;

    delete_message!(state, msg);
    delete_message!(state, new_msg);
    Ok(())
}

pub async fn week(bot: Bot, msg: Message, state: State) -> HandlerResult {
    let entries = get_week_timetable(&state.db, &msg.chat.id.to_string()).await?;
    let res = ui::timetable::week_view(entries);

    let new_msg = bot
        .send_message(msg.chat.id, res)
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .link_preview_options(DISABLED_LINK_PREVIEW_OPTIONS)
        .await?;

    delete_message!(state, msg);
    delete_message!(state, new_msg);
    Ok(())
}

pub async fn edit_timetable(bot: Bot, msg: Message, state: State) -> HandlerResult {
    let entries = get_full_timetable(&state.db, &msg.chat.id.to_string()).await?;
    let res = ui::timetable::edit_view(entries);

    let new_msg = bot
        .send_message(msg.chat.id, res)
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .link_preview_options(DISABLED_LINK_PREVIEW_OPTIONS)
        .await?;

    delete_message!(state, msg);
    delete_message!(state, new_msg);
    Ok(())
}

pub async fn now(bot: Bot, msg: Message, state: State) -> HandlerResult {
    let entry = get_current_entry(&state.db, &msg.chat.id.to_string()).await?;
    let res = ui::timetable::entry_view(entry.clone());
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
            .send_message(msg.chat.id, res)
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .reply_markup(InlineKeyboardMarkup::new(vec![vec![
                InlineKeyboardButton::url(inline_text, Url::parse(&inline_link).unwrap()),
            ]]))
            .await?;

        delete_message!(state, new_msg);
    } else {
        let new_msg = bot
            .send_message(msg.chat.id, res)
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .await?;
        delete_message!(state, new_msg);
    }

    delete_message!(state, msg);
    Ok(())
}

pub async fn next(bot: Bot, msg: Message, state: State) -> HandlerResult {
    let entry = get_next_entry(&state.db, &msg.chat.id.to_string()).await?;
    let res = ui::timetable::entry_view(entry.clone());
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
            .send_message(msg.chat.id, res)
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .reply_markup(InlineKeyboardMarkup::new(vec![vec![
                InlineKeyboardButton::url(inline_text, Url::parse(&inline_link).unwrap()),
            ]]))
            .await?;

        delete_message!(state, new_msg);
    } else {
        let new_msg = bot
            .send_message(msg.chat.id, res)
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .await?;
        delete_message!(state, new_msg);
    }
    delete_message!(state, msg);
    Ok(())
}

const DISABLED_LINK_PREVIEW_OPTIONS: LinkPreviewOptions = LinkPreviewOptions {
    is_disabled: true,
    url: None,
    prefer_large_media: false,
    prefer_small_media: false,
    show_above_text: false,
};
