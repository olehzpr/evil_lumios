use sea_orm::{entity::*, DatabaseConnection};
use teloxide::types::MessageId;

use crate::entities::gambles;

pub enum GambleType {
    Bet,
    Unknown,
}

impl From<GambleType> for String {
    fn from(gamble_type: GambleType) -> Self {
        match gamble_type {
            GambleType::Bet => "bet".to_string(),
            GambleType::Unknown => "unknown".to_string(),
        }
    }
}

impl From<&str> for GambleType {
    fn from(gamble_type: &str) -> Self {
        match gamble_type {
            "bet" => GambleType::Bet,
            _ => GambleType::Unknown,
        }
    }
}

pub struct GambleDto {
    pub user_id: i32,
    pub message_id: MessageId,
    pub is_win: bool,
    pub change: i32,
    pub bet: i32,
    pub gamble_type: GambleType,
}

pub async fn insert_gamble(conn: &DatabaseConnection, gamble: GambleDto) -> anyhow::Result<()> {
    let new_gamble = gambles::ActiveModel {
        user_id: Set(gamble.user_id),
        message_id: Set(gamble.message_id.to_string()),
        is_win: Set(gamble.is_win),
        change: Set(gamble.change),
        bet: Set(gamble.bet),
        gamble_type: Set(gamble.gamble_type.into()),
        ..Default::default()
    };

    new_gamble.insert(conn).await?;
    Ok(())
}
