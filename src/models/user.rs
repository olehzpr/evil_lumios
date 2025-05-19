use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, FromRow, Serialize, Deserialize)]
pub struct UserModel {
    pub id: i32,
    pub username: String,
    pub account_id: i64,
    pub chat_id: i64,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, FromRow)]
pub struct UserStatsModel {
    pub id: i32,
    pub user_id: i32,
    pub balance: i32,
    pub daily_limit: i32,
    pub daily_used: i32,
}
