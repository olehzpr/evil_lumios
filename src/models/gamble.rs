use teloxide::types::MessageId;

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct GambleDto {
    pub user_id: i32,
    pub message_id: MessageId,
    pub is_win: bool,
    pub change: i32,
    pub bet: i32,
    pub gamble_type: GambleType,
}
