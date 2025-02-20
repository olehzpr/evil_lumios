use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct GambleResult {
    id: i32,
    gamble_type: String,
    bet: i32,
    change: i32,
    is_win: bool,
}

#[derive(Serialize)]
pub struct Slot {}

#[derive(Serialize)]
pub struct Roulette {}

pub async fn slots() -> Json<GambleResult> {
    Json(GambleResult {
        id: 1,
        gamble_type: "slots".to_string(),
        bet: 10,
        change: 20,
        is_win: true,
    })
}

pub async fn roulette() -> Json<GambleResult> {
    Json(GambleResult {
        id: 1,
        gamble_type: "routette".to_string(),
        bet: 10,
        change: 20,
        is_win: true,
    })
}
