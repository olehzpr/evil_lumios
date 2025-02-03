use axum::Json;

#[derive(utoipa::ToSchema, serde::Serialize)]
pub struct User {
    id: i32,
    username: String,
    balance: i32,
}

#[derive(utoipa::ToSchema, serde::Serialize)]
pub struct Stats {
    users: Vec<User>,
}

#[derive(utoipa::ToSchema, serde::Serialize, utoipa::IntoParams)]
pub struct Pagination {
    offset: i32,
    limit: i32,
}

#[utoipa::path(
    get,
    path = "/stats",
    params(
        Pagination
    ),
    responses(
        (status = 200, description = "App is working", body = Stats),
    )
)]
pub async fn stats() -> Json<Stats> {
    Json(Stats {
        users: vec![
            User {
                id: 1,
                username: "user1".to_string(),
                balance: 100,
            },
            User {
                id: 2,
                username: "user2".to_string(),
                balance: 200,
            },
        ],
    })
}
