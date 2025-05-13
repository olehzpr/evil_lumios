use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, FromRow)]
pub struct QueueModel {
    pub id: i32,
    pub title: String,
    pub chat_id: String,
    pub message_id: String,
    pub is_mixed: Option<bool>,
    pub is_priority: bool,
    pub is_deleted: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, FromRow)]
pub struct QueueUserModel {
    pub id: i32,
    pub position: i32,
    pub priority: Option<i32>,
    pub is_frozen: Option<bool>,
    pub queue_id: i32,
    pub user_id: i32,
}

#[derive(Debug, Clone, PartialEq, FromRow)]
pub struct QueueUserWithUserModel {
    pub id: i32,
    pub position: i32,
    pub priority: Option<i32>,
    pub is_frozen: Option<bool>,
    pub queue_id: i32,
    pub user_id: i32,
    pub user_id_user: i32,
    pub username: String,
    pub account_id: String,
    pub chat_id_user: String,
    pub name: String,
}
