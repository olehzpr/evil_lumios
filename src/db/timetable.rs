use sea_orm::{entity::*, query::*, DatabaseConnection};
use serde_json::Value;

use crate::{
    bot::{
        timetable::{Day, Week},
        utils::time::get_current_time,
    },
    entities::{timetable_entries, timetables},
};

use crate::entities::timetable_entries::Entity as TimetableEntry;
use crate::entities::timetables::Entity as Timetable;

const OFFSET: chrono::Duration = chrono::Duration::minutes(5);

pub async fn import_timetable(
    conn: &DatabaseConnection,
    chat_id: &str,
    timetable: Value,
) -> anyhow::Result<()> {
    let existing_timetable = Timetable::find()
        .filter(timetables::Column::ChatId.eq(chat_id))
        .one(conn)
        .await?;

    if let Some(existing_timetable) = existing_timetable {
        TimetableEntry::delete_many()
            .filter(timetable_entries::Column::TimetableId.eq(existing_timetable.id))
            .exec(conn)
            .await?;
        Timetable::delete_many()
            .filter(timetables::Column::ChatId.eq(chat_id))
            .exec(conn)
            .await?;
    }

    let created_timetable = timetables::ActiveModel {
        chat_id: Set(chat_id.to_string()),
        ..Default::default()
    }
    .insert(conn)
    .await?;

    let mut entries: Vec<timetable_entries::ActiveModel> = vec![];
    for (week, schedule_key) in [
        (Week::First, "scheduleFirstWeek"),
        (Week::Second, "scheduleSecondWeek"),
    ] {
        if let Some(days) = timetable["data"][schedule_key].as_array() {
            for (index, day) in days.iter().enumerate() {
                if let Some(pairs) = day["pairs"].as_array() {
                    for entry in pairs {
                        entries.push(timetable_entries::ActiveModel {
                            timetable_id: Set(created_timetable.id),
                            week: Set(week as i32),
                            day: Set(index as i32),
                            class_name: Set(entry["name"].as_str().unwrap().to_string()),
                            class_type: Set(entry["tag"].as_str().unwrap().to_string()),
                            class_time: Set(chrono::NaiveTime::parse_from_str(
                                entry["time"].as_str().unwrap(),
                                "%H:%M",
                            )?),
                            link: Set(None),
                            ..Default::default()
                        });
                    }
                }
            }
        }
    }

    for entry in entries {
        entry.insert(conn).await?;
    }

    Ok(())
}

pub async fn get_today_timetable(
    conn: &DatabaseConnection,
    chat_id: &str,
) -> anyhow::Result<Vec<timetable_entries::Model>> {
    let week = Week::current();
    let day = Day::current();
    let entries = TimetableEntry::find()
        .join(
            JoinType::InnerJoin,
            timetable_entries::Relation::Timetables.def(),
        )
        .filter(timetables::Column::ChatId.eq(chat_id))
        .filter(timetable_entries::Column::Week.eq(week as i32))
        .filter(timetable_entries::Column::Day.eq(day as i32))
        .all(conn)
        .await?;

    Ok(entries)
}

pub async fn get_tomorrow_timetable(
    conn: &DatabaseConnection,
    chat_id: &str,
) -> anyhow::Result<Vec<timetable_entries::Model>> {
    let mut week = Week::current();
    let day = Day::current();
    if day == Day::Sat {
        week = week.next();
    }
    let day = day.next();

    let entries = TimetableEntry::find()
        .join(
            JoinType::InnerJoin,
            timetable_entries::Relation::Timetables.def(),
        )
        .filter(timetables::Column::ChatId.eq(chat_id))
        .filter(timetable_entries::Column::Week.eq(week as i32))
        .filter(timetable_entries::Column::Day.eq(day as i32))
        .all(conn)
        .await?;

    Ok(entries)
}

pub async fn get_week_timetable(
    conn: &DatabaseConnection,
    chat_id: &str,
) -> anyhow::Result<Vec<timetable_entries::Model>> {
    let week = Week::current();
    let entries = TimetableEntry::find()
        .join(
            JoinType::InnerJoin,
            timetable_entries::Relation::Timetables.def(),
        )
        .filter(timetables::Column::ChatId.eq(chat_id))
        .filter(timetable_entries::Column::Week.eq(week as i32))
        .order_by_asc(timetable_entries::Column::Day)
        .order_by_asc(timetable_entries::Column::ClassTime)
        .all(conn)
        .await?;

    Ok(entries)
}

pub async fn get_full_timetable(
    conn: &DatabaseConnection,
    chat_id: &str,
) -> anyhow::Result<Vec<timetable_entries::Model>> {
    let entries = TimetableEntry::find()
        .join(
            JoinType::InnerJoin,
            timetable_entries::Relation::Timetables.def(),
        )
        .filter(timetables::Column::ChatId.eq(chat_id))
        .order_by_asc(timetable_entries::Column::Week)
        .order_by_asc(timetable_entries::Column::Day)
        .order_by_asc(timetable_entries::Column::ClassTime)
        .all(conn)
        .await?;

    Ok(entries)
}

pub async fn get_current_entry(
    conn: &DatabaseConnection,
    chat_id: &str,
) -> anyhow::Result<Option<timetable_entries::Model>> {
    let week = Week::current();
    let day = Day::current();
    let now = get_current_time() - OFFSET;

    tracing::info!("Current time: {}", now.time());

    let entry = TimetableEntry::find()
        .join(
            JoinType::InnerJoin,
            timetable_entries::Relation::Timetables.def(),
        )
        .filter(timetables::Column::ChatId.eq(chat_id))
        .filter(timetable_entries::Column::Week.eq(week as i32))
        .filter(timetable_entries::Column::Day.eq(day as i32))
        .filter(timetable_entries::Column::ClassTime.lte(now.time()))
        .order_by_desc(timetable_entries::Column::ClassTime)
        .one(conn)
        .await?;

    Ok(entry)
}

pub async fn get_next_entry(
    conn: &DatabaseConnection,
    chat_id: &str,
) -> anyhow::Result<Option<timetable_entries::Model>> {
    let week = Week::current();
    let day = Day::current();
    let now = get_current_time() - OFFSET;

    let entry = TimetableEntry::find()
        .join(
            JoinType::InnerJoin,
            timetable_entries::Relation::Timetables.def(),
        )
        .filter(timetables::Column::ChatId.eq(chat_id))
        .filter(timetable_entries::Column::Week.eq(week as i32))
        .filter(timetable_entries::Column::Day.eq(day as i32))
        .filter(timetable_entries::Column::ClassTime.gte(now.time()))
        .order_by_asc(timetable_entries::Column::ClassTime)
        .one(conn)
        .await?;

    Ok(entry)
}

pub async fn get_entry_by_id(
    conn: &DatabaseConnection,
    entry_id: i32,
) -> anyhow::Result<Option<timetable_entries::Model>> {
    let entry = TimetableEntry::find_by_id(entry_id).one(conn).await?;

    Ok(entry)
}
pub async fn update_link(
    conn: &DatabaseConnection,
    entry_id: i32,
    link: &str,
) -> anyhow::Result<()> {
    let entry: Option<timetable_entries::ActiveModel> = TimetableEntry::find_by_id(entry_id)
        .one(conn)
        .await?
        .map(Into::into);

    if let Some(mut entry) = entry {
        entry.link = Set(Some(link.to_string()));
        entry.update(conn).await?;
    }

    Ok(())
}
