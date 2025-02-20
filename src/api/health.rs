use axum::Json;

#[derive(serde::Serialize)]
pub struct Health {
    status: String,
}

pub async fn check_health() -> Json<Health> {
    Json(Health {
        status: "ok".to_string(),
    })
}
