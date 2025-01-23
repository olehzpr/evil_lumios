use std::vec;

use diesel::{ExpressionMethods, JoinOnDsl, OptionalExtension, QueryDsl, RunQueryDsl};
use evil_lumios::Event;
use teloxide::{types::Message, Bot};

use crate::{
    bot::{
        externsions::{ExtendedBot, Msg},
        ui::{self, extedned_timetable_entry_view},
        utils::params::get_param,
    },
    db::{
        connection,
        models::{NewTimetable, NewTimetableEntry, Timetable, TimetableEntry},
    },
    schema, State,
};

use super::{
    utils::{get_current_day, get_current_week, get_day_name},
    HandlerResult,
};

pub async fn import(bot: Bot, msg: Message, state: State) -> HandlerResult {
    let conn = &mut connection(&state).await;
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

    bot.send_extended(Msg::Regular(msg.chat, "Розклад успішно імпортований ✅"))
        .await?;

    Ok(())
}

pub async fn today(bot: Bot, msg: Message, state: State) -> HandlerResult {
    let current_week = get_current_week();
    let current_day = get_current_day();
    let conn = &mut connection(&state).await;
    let entries = schema::timetable_entries::table
        .inner_join(
            schema::timetables::table
                .on(schema::timetable_entries::timetable_id.eq(schema::timetables::id)),
        )
        .filter(schema::timetables::chat_id.eq(&msg.chat.id.to_string()))
        .filter(schema::timetable_entries::week.eq(current_week as i32))
        .filter(schema::timetable_entries::day.eq(current_day as i32))
        .select(schema::timetable_entries::all_columns)
        .load::<TimetableEntry>(conn)?;
    let mut response = String::new();
    for entry in entries {
        response.push_str(&ui::small_timetable_entry_view(&entry, false));
    }
    state.sender.send(Event::DeleteMessage {
        chat_id: msg.chat.id,
        message_id: msg.id,
    })?;
    bot.send_extended(Msg::Temp(msg.chat, &response, state.sender.clone()))
        .await?;
    Ok(())
}

pub async fn tomorrow(bot: Bot, msg: Message, state: State) -> HandlerResult {
    let current_week = get_current_week();
    let next_day = (get_current_day() + 1) % 7;
    let conn = &mut connection(&state).await;
    let entries = schema::timetable_entries::table
        .inner_join(
            schema::timetables::table
                .on(schema::timetable_entries::timetable_id.eq(schema::timetables::id)),
        )
        .filter(schema::timetables::chat_id.eq(&msg.chat.id.to_string()))
        .filter(schema::timetable_entries::week.eq(current_week as i32))
        .filter(schema::timetable_entries::day.eq(next_day as i32))
        .select(schema::timetable_entries::all_columns)
        .load::<TimetableEntry>(conn)?;
    let mut response = String::new();
    for entry in entries {
        response.push_str(&ui::small_timetable_entry_view(&entry, false));
    }
    if response.is_empty() {
        response = "No classes tomorrow".to_string();
    }
    state.sender.send(Event::DeleteMessage {
        chat_id: msg.chat.id,
        message_id: msg.id,
    })?;
    bot.send_extended(Msg::Temp(msg.chat, &response, state.sender.clone()))
        .await?;
    Ok(())
}

pub async fn week(bot: Bot, msg: Message, state: State) -> HandlerResult {
    let conn = &mut connection(&state).await;
    let entries = schema::timetable_entries::table
        .inner_join(
            schema::timetables::table
                .on(schema::timetable_entries::timetable_id.eq(schema::timetables::id)),
        )
        .filter(schema::timetables::chat_id.eq(&msg.chat.id.to_string()))
        .filter(schema::timetable_entries::week.eq(get_current_week() as i32))
        .select(schema::timetable_entries::all_columns)
        .order((
            schema::timetable_entries::day.asc(),
            schema::timetable_entries::class_time.asc(),
        ))
        .load::<TimetableEntry>(conn)?;
    let mut response = String::new();
    let mut day: u8 = 0;
    for entry in entries {
        if entry.day == day as i32 {
            response.push_str(&format!("\n*{}*\n", get_day_name(day)));
            day += 1;
        }
        response.push_str(&ui::small_timetable_entry_view(&entry, false));
    }
    state.sender.send(Event::DeleteMessage {
        chat_id: msg.chat.id,
        message_id: msg.id,
    })?;
    bot.send_extended(Msg::Temp(msg.chat, &response, state.sender.clone()))
        .await?;
    Ok(())
}

pub async fn edit_timetable(bot: Bot, msg: Message, state: State) -> HandlerResult {
    let conn = &mut connection(&state).await;
    let entries = schema::timetable_entries::table
        .inner_join(
            schema::timetables::table
                .on(schema::timetable_entries::timetable_id.eq(schema::timetables::id)),
        )
        .filter(schema::timetables::chat_id.eq(&msg.chat.id.to_string()))
        .select(schema::timetable_entries::all_columns)
        .order((
            schema::timetable_entries::week.asc(),
            schema::timetable_entries::day.asc(),
            schema::timetable_entries::class_time.asc(),
        ))
        .load::<TimetableEntry>(conn)?;
    let mut response = String::new();
    let mut day: u8 = 0;
    let mut week: u8 = 1;
    for entry in entries {
        if entry.week == week as i32 {
            response.push_str(&format!("\n*📅 Week {}*\n", week));
            week += 1;
            day = 0;
        }
        if entry.day == day as i32 {
            response.push_str(&format!("\n*{}*\n", get_day_name(day)));
            day += 1;
        }
        response.push_str(&ui::small_timetable_entry_view(&entry, true));
    }
    bot.send_extended(Msg::Regular(msg.chat, &response)).await?;
    Ok(())
}

pub async fn now(bot: Bot, msg: Message, state: State) -> HandlerResult {
    let current_week = get_current_week();
    let current_day = get_current_day();
    let current_time = chrono::Utc::now().time();
    let conn = &mut connection(&state).await;
    let entry = schema::timetable_entries::table
        .inner_join(
            schema::timetables::table
                .on(schema::timetable_entries::timetable_id.eq(schema::timetables::id)),
        )
        .filter(schema::timetables::chat_id.eq(&msg.chat.id.to_string()))
        .filter(schema::timetable_entries::week.eq(current_week as i32))
        .filter(schema::timetable_entries::day.eq(current_day as i32))
        .filter(schema::timetable_entries::class_time.ge(current_time))
        .select(schema::timetable_entries::all_columns)
        .order(schema::timetable_entries::class_time.asc())
        .first::<TimetableEntry>(conn)
        .optional()?;
    match entry {
        Some(entry) => {
            bot.send_extended(Msg::Regular(
                msg.chat,
                &extedned_timetable_entry_view(&entry),
            ))
            .await?;
        }
        None => {
            bot.send_extended(Msg::Regular(msg.chat, "На сьогодні заняття закінчились"))
                .await?;
        }
    }
    Ok(())
}

pub async fn next(bot: Bot, msg: Message, state: State) -> HandlerResult {
    let current_week = get_current_week();
    let current_day = get_current_day();
    let current_time = chrono::Utc::now().time();
    let conn = &mut connection(&state).await;
    let entry = schema::timetable_entries::table
        .inner_join(
            schema::timetables::table
                .on(schema::timetable_entries::timetable_id.eq(schema::timetables::id)),
        )
        .filter(schema::timetables::chat_id.eq(&msg.chat.id.to_string()))
        .filter(schema::timetable_entries::week.eq(current_week as i32))
        .filter(schema::timetable_entries::day.eq(current_day as i32))
        .filter(schema::timetable_entries::class_time.ge(current_time))
        .select(schema::timetable_entries::all_columns)
        .order(schema::timetable_entries::class_time.asc())
        .offset(1)
        .first::<TimetableEntry>(conn)
        .optional()?;
    match entry {
        Some(entry) => {
            bot.send_extended(Msg::Regular(
                msg.chat,
                &extedned_timetable_entry_view(&entry),
            ))
            .await?;
        }
        None => {
            bot.send_extended(Msg::Regular(msg.chat, "На сьогодні заняття закінчились"))
                .await?;
        }
    }

    Ok(())
}
