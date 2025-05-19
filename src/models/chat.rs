use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, FromRow, Serialize, Deserialize)]
pub struct ChatModel {
    pub id: i32,
    pub chat_id: i64,
    pub title: String,
    pub description: Option<String>,
}
