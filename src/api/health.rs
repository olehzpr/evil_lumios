use axum::Json;

#[derive(utoipa::ToSchema, serde::Serialize)]
pub struct Health {
    status: String,
}

#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "App is working", body = Health),
    )
)]
pub async fn check_health() -> Json<Health> {
    Json(Health {
        status: "ok".to_string(),
    })
}
