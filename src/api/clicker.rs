use axum::Json;

#[derive(utoipa::ToSchema, serde::Serialize)]
pub struct ClickResponse {
    status: String,
    available: i32,
}

#[derive(utoipa::ToSchema, serde::Serialize, utoipa::IntoParams)]
pub struct UserId {
    id: i32,
}

#[utoipa::path(
    put,
    path = "/clicker",
    params(
        UserId
    ),
    responses(
        (status = 200, description = "Click for points", body = ClickResponse),
    )
)]
pub async fn click() -> Json<ClickResponse> {
    Json(ClickResponse {
        available: 100,
        status: "success".to_string(),
    })
}
