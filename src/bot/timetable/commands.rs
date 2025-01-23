use std::vec;

use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};
use evil_lumios::Event;
use teloxide::{types::Message, Bot};

use crate::{
    bot::{
        externsions::{ExtendedBot, Msg},
        ui::{self},
        utils::params::get_param,
    },
    db::{
        models::{NewTimetable, NewTimetableEntry, Timetable},
        timetable::{
            get_current_entry, get_full_timetable, get_next_entry, get_today_timetable,
            get_tomorrow_timetable, get_week_timetable,
        },
        StateWithConnection,
    },
    schema, send_autodelete, send_message, State,
};

use super::HandlerResult;

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

    let existing_timetable = schema::timetables::table
        .filter(schema::timetables::chat_id.eq(&msg.chat.id.to_string()))
        .first::<Timetable>(conn)
        .optional()?;
    if let Some(existing_timetable) = existing_timetable {
        diesel::delete(schema::timetable_entries::table)
            .filter(schema::timetable_entries::timetable_id.eq(existing_timetable.id))
            .execute(conn)?;
        diesel::delete(schema::timetables::table)
            .filter(schema::timetables::chat_id.eq(&msg.chat.id.to_string()))
            .execute(conn)?;
    }

    let created_timetable = diesel::insert_into(schema::timetables::table)
        .values(NewTimetable {
            chat_id: &msg.chat.id.to_string(),
        })
        .get_result::<Timetable>(conn)?;
    let mut entries: Vec<NewTimetableEntry> = vec![];
    for (week, schedule_key) in [(1, "scheduleFirstWeek"), (2, "scheduleSecondWeek")] {
        if let Some(days) = timetable["data"][schedule_key].as_array() {
            for (index, day) in days.iter().enumerate() {
                if let Some(pairs) = day["pairs"].as_array() {
                    for entry in pairs {
                        entries.push(NewTimetableEntry {
                            timetable_id: created_timetable.id,
                            week,
                            day: index as i32,
                            class_name: entry["name"].as_str().unwrap(),
                            class_type: entry["tag"].as_str().unwrap(),
                            class_time: chrono::NaiveTime::parse_from_str(
                                entry["time"].as_str().unwrap(),
                                "%H:%M",
                            )
                            .unwrap(),
                            link: None,
                        });
                    }
                }
            }
        }
    }
    entries.iter().for_each(|entry| {
        diesel::insert_into(schema::timetable_entries::table)
            .values(entry)
            .execute(conn)
            .unwrap();
    });

    send_message!(bot, msg, "Розклад успішно імпортовано ✅");
    Ok(())
}

pub async fn today(bot: Bot, msg: Message, state: State) -> HandlerResult {
    let conn = &mut state.conn().await;
    let entries = get_today_timetable(conn, &msg.chat.id.to_string()).await?;
    let mut res = ui::timetable::day_view(entries);
    if res.is_empty() {
        res = "Сьогодні немає жодних пар. Можна відпочивати 🔥".to_string();
    }
    send_autodelete!(bot, msg, state, &res);
    Ok(())
}

pub async fn tomorrow(bot: Bot, msg: Message, state: State) -> HandlerResult {
    let conn = &mut state.conn().await;
    let entries = get_tomorrow_timetable(conn, &msg.chat.id.to_string()).await?;
    let mut res = ui::timetable::day_view(entries);
    if res.is_empty() {
        res = "Завтра немає жодних пар. Можна відпочивати 🦅".to_string();
    }
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
    let response = ui::timetable::edit_view(entries);
    send_message!(bot, msg, &response);
    Ok(())
}

pub async fn now(bot: Bot, msg: Message, state: State) -> HandlerResult {
    let conn = &mut state.conn().await;
    let entry = get_current_entry(conn, &msg.chat.id.to_string()).await?;
    let res = match entry {
        Some(entry) => ui::timetable::entry_view(&entry),
        None => "Наразі заняття відсутні 😎".to_string(),
    };
    send_autodelete!(bot, msg, state, &res);
    Ok(())
}

pub async fn next(bot: Bot, msg: Message, state: State) -> HandlerResult {
    let conn = &mut state.conn().await;
    let entry = get_next_entry(conn, &msg.chat.id.to_string()).await?;
    let res = match entry {
        Some(entry) => ui::timetable::entry_view(&entry),
        None => "Наразі заняття відсутні 😎".to_string(),
    };
    send_autodelete!(bot, msg, state, &res);
    Ok(())
}
