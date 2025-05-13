use chrono::NaiveTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, FromRow)]
pub struct TimetableModel {
    pub id: i32,
    pub chat_id: String,
}

#[derive(Debug, Clone, PartialEq, FromRow, Serialize, Deserialize)]
pub struct TimetableEntryModel {
    pub id: i32,
    pub week: i32,
    pub day: i32,
    pub timetable_id: i32,
    pub class_name: String,
    pub class_type: String,
    pub class_time: NaiveTime,
    pub link: Option<String>,
}
