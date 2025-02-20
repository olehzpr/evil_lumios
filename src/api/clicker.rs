use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct ClickResponse {
    status: String,
    available: i32,
}

#[derive(Serialize)]
pub struct UserId {
    id: i32,
}

pub async fn click() -> Json<ClickResponse> {
    Json(ClickResponse {
        available: 100,
        status: "success".to_string(),
    })
}
