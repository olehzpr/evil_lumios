use crate::{bot::handler::HandlerResult, state::Event};
use teloxide::{types::Message, Bot};

use crate::{
    bot::{
        externsions::{ExtendedBot, Msg},
        ui::{self},
        utils::params::get_param,
    },
    db::{
        timetable::{
            get_current_entry, get_full_timetable, get_next_entry, get_today_timetable,
            get_tomorrow_timetable, get_week_timetable, import_timetable,
        },
        StateWithConnection,
    },
    send_autodelete, send_message, State,
};

pub async fn import(bot: Bot, msg: Message, state: State) -> HandlerResult {
    let conn = &mut state.conn().await;
    let group_name = get_param(
        &msg,
        "Ви повинні вказати код групи. Наприклад /import ІП-32",
    )?;
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

    import_timetable(conn, &msg.chat.id.to_string(), timetable).await?;

    send_message!(bot, msg, "Розклад успішно імпортовано ✅");
    Ok(())
}

pub async fn today(bot: Bot, msg: Message, state: State) -> HandlerResult {
    let conn = &mut state.conn().await;
    let entries = get_today_timetable(conn, &msg.chat.id.to_string()).await?;
    let res = ui::timetable::day_view(entries);
    send_autodelete!(bot, msg, state, &res);
    Ok(())
}

pub async fn tomorrow(bot: Bot, msg: Message, state: State) -> HandlerResult {
    let conn = &mut state.conn().await;
    let entries = get_tomorrow_timetable(conn, &msg.chat.id.to_string()).await?;
    let res = ui::timetable::day_view(entries);
    send_autodelete!(bot, msg, state, &res);
    Ok(())
}

pub async fn week(bot: Bot, msg: Message, state: State) -> HandlerResult {
    let conn = &mut state.conn().await;
    let entries = get_week_timetable(conn, &msg.chat.id.to_string()).await?;
    let res = ui::timetable::week_view(entries);
    send_autodelete!(bot, msg, state, &res);
    Ok(())
}

pub async fn edit_timetable(bot: Bot, msg: Message, state: State) -> HandlerResult {
    let conn = &mut state.conn().await;
    let entries = get_full_timetable(conn, &msg.chat.id.to_string()).await?;
    let res = ui::timetable::edit_view(entries);
    send_message!(bot, msg, &res);
    Ok(())
}

pub async fn now(bot: Bot, msg: Message, state: State) -> HandlerResult {
    let conn = &mut state.conn().await;
    let entry = get_current_entry(conn, &msg.chat.id.to_string()).await?;
    let res = ui::timetable::entry_view(entry);
    send_autodelete!(bot, msg, state, &res);
    Ok(())
}

pub async fn next(bot: Bot, msg: Message, state: State) -> HandlerResult {
    let conn = &mut state.conn().await;
    let entry = get_next_entry(conn, &msg.chat.id.to_string()).await?;
    let res = ui::timetable::entry_view(entry);
    send_autodelete!(bot, msg, state, &res);
    Ok(())
}
