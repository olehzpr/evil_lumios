use crate::schema::*;
use diesel::{pg::Pg, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, QueryableByName, Debug, Serialize, Deserialize)]
#[diesel(table_name = chats)]
#[diesel(check_for_backend(Pg))]
pub struct Chat {
    pub id: i32,
    pub chat_id: String,
    pub group_id: Option<String>,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = chats)]
pub struct NewChat<'a> {
    pub chat_id: &'a str,
    pub group_id: Option<&'a str>,
    pub title: &'a str,
    pub description: Option<&'a str>,
}

#[derive(Queryable, Selectable, QueryableByName, Debug, Serialize, Deserialize, Clone)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub account_id: String,
    pub chat_id: String,
    pub name: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub account_id: &'a str,
    pub chat_id: &'a str,
    pub name: &'a str,
}

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = user_stats)]
#[diesel(check_for_backend(Pg))]
pub struct UserStats {
    pub id: i32,
    pub user_id: i32,
    pub balance: i32,
    pub daily_limit: i32,
    pub daily_used: i32,
}

#[derive(Insertable)]
#[diesel(table_name = user_stats)]
pub struct NewUserStats {
    pub user_id: i32,
}

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = gambles)]
#[diesel(check_for_backend(Pg))]
pub struct Gamble {
    pub id: i32,
    pub user_id: i32,
    pub message_id: String,
    pub gamble_type: String,
    pub bet: i32,
    pub change: i32,
    pub is_win: bool,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = gambles)]
pub struct NewGamble {
    pub user_id: i32,
    pub message_id: String,
    pub gamble_type: String,
    pub bet: i32,
    pub change: i32,
    pub is_win: bool,
}

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = timetables)]
#[diesel(check_for_backend(Pg))]
pub struct Timetable {
    pub id: i32,
    pub chat_id: String,
}

#[derive(Insertable)]
#[diesel(table_name = timetables)]
pub struct NewTimetable<'a> {
    pub chat_id: &'a str,
}

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize, Clone)]
#[diesel(table_name = timetable_entries)]
#[diesel(check_for_backend(Pg))]
pub struct TimetableEntry {
    pub id: i32,
    pub week: i32,
    pub day: i32,
    pub timetable_id: i32,
    pub class_name: String,
    pub class_type: String,
    pub class_time: chrono::NaiveTime,
    pub link: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = timetable_entries)]
pub struct NewTimetableEntry<'a> {
    pub week: i32,
    pub day: i32,
    pub timetable_id: i32,
    pub class_name: &'a str,
    pub class_type: &'a str,
    pub class_time: chrono::NaiveTime,
    pub link: Option<&'a str>,
}

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = queues)]
#[diesel(check_for_backend(Pg))]
pub struct Queue {
    pub id: i32,
    pub title: String,
    pub chat_id: String,
    pub message_id: String,
    pub is_mixed: Option<bool>,
    pub is_deleted: bool,
    pub is_priority: bool,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = queues)]
pub struct NewQueue<'a> {
    pub title: &'a str,
    pub chat_id: &'a str,
    pub message_id: &'a str,
    pub is_mixed: Option<bool>,
    pub is_priority: bool,
}

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = queue_users)]
#[diesel(check_for_backend(Pg))]
pub struct QueueUser {
    pub id: i32,
    pub position: i32,
    pub priority: Option<i32>,
    pub is_freezed: Option<bool>,
    pub queue_id: i32,
    pub user_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = queue_users)]
pub struct NewQueueUser {
    pub queue_id: i32,
    pub user_id: i32,
    pub position: i32,
    pub priority: Option<i32>,
}
