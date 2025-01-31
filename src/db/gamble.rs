use diesel::{PgConnection, RunQueryDsl};
use teloxide::types::MessageId;

use super::models::NewGamble;

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

pub async fn insert_gamble(conn: &mut PgConnection, gamble: GambleDto) -> anyhow::Result<()> {
    diesel::insert_into(crate::schema::gambles::table)
        .values(NewGamble {
            user_id: gamble.user_id,
            message_id: gamble.message_id.to_string(),
            is_win: gamble.is_win,
            change: gamble.change,
            bet: gamble.bet,
            gamble_type: gamble.gamble_type.into(),
        })
        .execute(conn)?;
    Ok(())
}
