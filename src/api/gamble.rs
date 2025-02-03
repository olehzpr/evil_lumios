use axum::Json;

#[derive(utoipa::ToSchema, serde::Serialize)]
pub struct GambleResult {
    id: i32,
    gamble_type: String,
    bet: i32,
    change: i32,
    is_win: bool,
}

#[derive(utoipa::ToSchema, serde::Serialize)]
pub struct Slot {}

#[derive(utoipa::ToSchema, serde::Serialize)]
pub struct Roulette {}

#[utoipa::path(
    post,
    path = "/slots",
    request_body = Slot,
    responses(
        (status = 200, description = "Gamble Result", body = GambleResult)
    )
)]
pub async fn slots() -> Json<GambleResult> {
    Json(GambleResult {
        id: 1,
        gamble_type: "slots".to_string(),
        bet: 10,
        change: 20,
        is_win: true,
    })
}

#[utoipa::path(
    post,
    path = "/routette",
    request_body = Roulette,
    responses(
        (status = 200, description = "Gamble Result", body = GambleResult)
    )
)]
pub async fn roulette() -> Json<GambleResult> {
    Json(GambleResult {
        id: 1,
        gamble_type: "routette".to_string(),
        bet: 10,
        change: 20,
        is_win: true,
    })
}
