use anyhow::Context;
use chrono::NaiveTime;
use serde_json::Value;
use sqlx::{PgPool, Row};

use crate::{
    bot::{
        timetable::{Day, Week},
        utils::time::get_current_time,
    },
    models::timetable::{TimetableEntryModel, TimetableModel},
};

const OFFSET: chrono::Duration = chrono::Duration::minutes(5);

pub async fn import_timetable(pool: &PgPool, chat_id: i64, timetable: Value) -> anyhow::Result<()> {
    let mut tx = pool.begin().await.context("Failed to begin transaction")?;

    let existing_timetable = sqlx::query(
        r#"
        SELECT id, chat_id
        FROM timetables
        WHERE chat_id = $1
        "#,
    )
    .bind(chat_id)
    .fetch_optional(&mut *tx)
    .await
    .context("Failed to query existing timetable")?
    .map(|row| TimetableModel {
        id: row.get("id"),
        chat_id: row.get("chat_id"),
    });

    if let Some(existing_timetable) = existing_timetable {
        sqlx::query(
            r#"
            DELETE FROM timetable_entries
            WHERE timetable_id = $1
            "#,
        )
        .bind(existing_timetable.id)
        .execute(&mut *tx)
        .await
        .context("Failed to delete existing timetable entries")?;

        sqlx::query(
            r#"
            DELETE FROM timetables
            WHERE id = $1
            "#,
        )
        .bind(existing_timetable.id)
        .execute(&mut *tx)
        .await
        .context("Failed to delete existing timetable")?;
    }

    let created_timetable = sqlx::query(
        r#"
        INSERT INTO timetables (chat_id)
        VALUES ($1)
        RETURNING id, chat_id
        "#,
    )
    .bind(chat_id)
    .fetch_one(&mut *tx)
    .await
    .context("Failed to insert new timetable")?;

    let created_timetable = TimetableModel {
        id: created_timetable.get("id"),
        chat_id: created_timetable.get("chat_id"),
    };

    for (week, schedule_key) in [
        (Week::First, "scheduleFirstWeek"),
        (Week::Second, "scheduleSecondWeek"),
    ] {
        if let Some(days) = timetable["data"][schedule_key].as_array() {
            for (index, day) in days.iter().enumerate() {
                if let Some(pairs) = day["pairs"].as_array() {
                    for entry in pairs {
                        let class_time_str = entry["time"].as_str().ok_or_else(|| {
                            anyhow::anyhow!("Missing 'time' field in timetable entry")
                        })?;
                        let class_time = NaiveTime::parse_from_str(class_time_str, "%H:%M")
                            .context(format!("Failed to parse time '{}'", class_time_str))?;

                        sqlx::query(
                            r#"
                            INSERT INTO timetable_entries (timetable_id, week, day, class_name, class_type, class_time, link)
                            VALUES ($1, $2, $3, $4, $5, $6, $7)
                            "#,
                        )
                        .bind(created_timetable.id)
                        .bind(week as i32)
                        .bind(index as i32)
                        .bind(entry["name"].as_str().unwrap_or_default())
                        .bind(entry["tag"].as_str().unwrap_or_default())
                        .bind(class_time)
                        .bind(None::<String>) // Link is initially null
                        .execute(&mut *tx)
                        .await
                        .context("Failed to insert timetable entry")?;
                    }
                }
            }
        }
    }

    tx.commit().await.context("Failed to commit transaction")?;

    Ok(())
}

pub async fn get_today_timetable(
    pool: &PgPool,
    chat_id: &str,
) -> anyhow::Result<Vec<TimetableEntryModel>> {
    let week = Week::current();
    let day = Day::current();

    let entries = sqlx::query(
        r#"
        SELECT
            te.id,
            te.week,
            te.day,
            te.timetable_id,
            te.class_name,
            te.class_type,
            te.class_time,
            te.link
        FROM timetable_entries te
        JOIN timetables tt ON te.timetable_id = tt.id
        WHERE tt.chat_id = $1
          AND te.week = $2
          AND te.day = $3
        ORDER BY te.class_time
        "#,
    )
    .bind(chat_id)
    .bind(week as i32)
    .bind(day as i32)
    .fetch_all(pool)
    .await
    .context("Failed to query today's timetable")?
    .into_iter()
    .map(|row| TimetableEntryModel {
        id: row.get("id"),
        week: row.get("week"),
        day: row.get("day"),
        timetable_id: row.get("timetable_id"),
        class_name: row.get("class_name"),
        class_type: row.get("class_type"),
        class_time: row.get("class_time"),
        link: row.get("link"),
    })
    .collect();

    Ok(entries)
}

pub async fn get_tomorrow_timetable(
    pool: &PgPool,
    chat_id: &str,
) -> anyhow::Result<Vec<TimetableEntryModel>> {
    let mut week = Week::current();
    let day = Day::current();
    if day == Day::Sat {
        week = week.next();
    }
    let next_day = day.next();

    let entries = sqlx::query(
        r#"
        SELECT
            te.id,
            te.week,
            te.day,
            te.timetable_id,
            te.class_name,
            te.class_type,
            te.class_time,
            te.link
        FROM timetable_entries te
        JOIN timetables tt ON te.timetable_id = tt.id
        WHERE tt.chat_id = $1
          AND te.week = $2
          AND te.day = $3
        ORDER BY te.class_time
        "#,
    )
    .bind(chat_id)
    .bind(week as i32)
    .bind(next_day as i32)
    .fetch_all(pool)
    .await
    .context("Failed to query tomorrow's timetable")?
    .into_iter()
    .map(|row| TimetableEntryModel {
        id: row.get("id"),
        week: row.get("week"),
        day: row.get("day"),
        timetable_id: row.get("timetable_id"),
        class_name: row.get("class_name"),
        class_type: row.get("class_type"),
        class_time: row.get("class_time"),
        link: row.get("link"),
    })
    .collect();

    Ok(entries)
}

pub async fn get_week_timetable(
    pool: &PgPool,
    chat_id: &str,
) -> anyhow::Result<Vec<TimetableEntryModel>> {
    let week = Week::current();

    let entries = sqlx::query(
        r#"
        SELECT
            te.id,
            te.week,
            te.day,
            te.timetable_id,
            te.class_name,
            te.class_type,
            te.class_time,
            te.link
        FROM timetable_entries te
        JOIN timetables tt ON te.timetable_id = tt.id
        WHERE tt.chat_id = $1
          AND te.week = $2
        ORDER BY te.day, te.class_time
        "#,
    )
    .bind(chat_id)
    .bind(week as i32)
    .fetch_all(pool)
    .await
    .context("Failed to query week's timetable")?
    .into_iter()
    .map(|row| TimetableEntryModel {
        id: row.get("id"),
        week: row.get("week"),
        day: row.get("day"),
        timetable_id: row.get("timetable_id"),
        class_name: row.get("class_name"),
        class_type: row.get("class_type"),
        class_time: row.get("class_time"),
        link: row.get("link"),
    })
    .collect();

    Ok(entries)
}

pub async fn get_full_timetable(
    pool: &PgPool,
    chat_id: &str,
) -> anyhow::Result<Vec<TimetableEntryModel>> {
    let entries = sqlx::query(
        r#"
        SELECT
            te.id,
            te.week,
            te.day,
            te.timetable_id,
            te.class_name,
            te.class_type,
            te.class_time,
            te.link
        FROM timetable_entries te
        JOIN timetables tt ON te.timetable_id = tt.id
        WHERE tt.chat_id = $1
        ORDER BY te.week, te.day, te.class_time
        "#,
    )
    .bind(chat_id)
    .fetch_all(pool)
    .await
    .context("Failed to query full timetable")?
    .into_iter()
    .map(|row| TimetableEntryModel {
        id: row.get("id"),
        week: row.get("week"),
        day: row.get("day"),
        timetable_id: row.get("timetable_id"),
        class_name: row.get("class_name"),
        class_type: row.get("class_type"),
        class_time: row.get("class_time"),
        link: row.get("link"),
    })
    .collect();

    Ok(entries)
}

pub async fn get_current_entry(
    pool: &PgPool,
    chat_id: &str,
) -> anyhow::Result<Option<TimetableEntryModel>> {
    let week = Week::current();
    let day = Day::current();
    let now = get_current_time() - OFFSET;
    let now_time = now.time();

    tracing::info!("Current time for timetable lookup: {}", now_time);

    let entry = sqlx::query(
        r#"
        SELECT
            te.id,
            te.week,
            te.day,
            te.timetable_id,
            te.class_name,
            te.class_type,
            te.class_time,
            te.link
        FROM timetable_entries te
        JOIN timetables tt ON te.timetable_id = tt.id
        WHERE tt.chat_id = $1
          AND te.week = $2
          AND te.day = $3
          AND te.class_time <= $4
        ORDER BY te.class_time DESC
        LIMIT 1
        "#,
    )
    .bind(chat_id)
    .bind(week as i32)
    .bind(day as i32)
    .bind(now_time)
    .fetch_optional(pool)
    .await
    .context("Failed to query current timetable entry")?
    .map(|row| TimetableEntryModel {
        id: row.get("id"),
        week: row.get("week"),
        day: row.get("day"),
        timetable_id: row.get("timetable_id"),
        class_name: row.get("class_name"),
        class_type: row.get("class_type"),
        class_time: row.get("class_time"),
        link: row.get("link"),
    });

    Ok(entry)
}

pub async fn get_next_entry(
    pool: &PgPool,
    chat_id: &str,
) -> anyhow::Result<Option<TimetableEntryModel>> {
    let week = Week::current();
    let day = Day::current();
    let now = get_current_time() - OFFSET;
    let now_time = now.time();

    let entry = sqlx::query(
        r#"
        SELECT
            te.id,
            te.week,
            te.day,
            te.timetable_id,
            te.class_name,
            te.class_type,
            te.class_time,
            te.link
        FROM timetable_entries te
        JOIN timetables tt ON te.timetable_id = tt.id
        WHERE tt.chat_id = $1
          AND te.week = $2
          AND te.day = $3
          AND te.class_time >= $4
        ORDER BY te.class_time ASC
        LIMIT 1
        "#,
    )
    .bind(chat_id)
    .bind(week as i32)
    .bind(day as i32)
    .bind(now_time)
    .fetch_optional(pool)
    .await
    .context("Failed to query next timetable entry")?
    .map(|row| TimetableEntryModel {
        id: row.get("id"),
        week: row.get("week"),
        day: row.get("day"),
        timetable_id: row.get("timetable_id"),
        class_name: row.get("class_name"),
        class_type: row.get("class_type"),
        class_time: row.get("class_time"),
        link: row.get("link"),
    });

    Ok(entry)
}

pub async fn get_entry_by_id(
    pool: &PgPool,
    entry_id: i32,
) -> anyhow::Result<Option<TimetableEntryModel>> {
    let entry = sqlx::query(
        r#"
        SELECT
            id,
            week,
            day,
            timetable_id,
            class_name,
            class_type,
            class_time,
            link
        FROM timetable_entries
        WHERE id = $1
        "#,
    )
    .bind(entry_id)
    .fetch_optional(pool)
    .await
    .context(format!(
        "Failed to query timetable entry by id: {}",
        entry_id
    ))?
    .map(|row| TimetableEntryModel {
        id: row.get("id"),
        week: row.get("week"),
        day: row.get("day"),
        timetable_id: row.get("timetable_id"),
        class_name: row.get("class_name"),
        class_type: row.get("class_type"),
        class_time: row.get("class_time"),
        link: row.get("link"),
    });

    Ok(entry)
}

pub async fn update_link(pool: &PgPool, entry_id: i32, link: &str) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        UPDATE timetable_entries
        SET link = $1
        WHERE id = $2
        "#,
    )
    .bind(link)
    .bind(entry_id)
    .execute(pool)
    .await
    .context(format!("Failed to update link for entry id: {}", entry_id))?;

    Ok(())
}
