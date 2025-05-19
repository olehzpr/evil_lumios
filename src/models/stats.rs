use chrono::NaiveDateTime;
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, PartialEq, FromRow)]
pub struct GambleModel {
    pub id: i32,
    pub user_id: i32,
    pub message_id: i32,
    pub gamble_type: String,
    pub bet: i32,
    pub change: i32,
    pub is_win: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, FromRow)]
pub struct GroupMemberStat {
    pub username: String,
    pub balance: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FullStats {
    pub user_id: i32,
    pub balance: i32,
    pub daily_limit: i32,
    pub daily_used: i32,
    pub num_of_wins: i32,
    pub num_of_losses: i32,
    pub total_won: i32,
    pub total_lost: i32,
    pub total_gambles: i32,
    pub longest_winning_streak: i32,
    pub longest_losing_streak: i32,
    pub current_streak: i32,
    pub average_bet: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroupStats {
    pub group_name: String,
    pub stats: Vec<GroupMemberStat>,
}
