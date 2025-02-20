use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    id: i32,
    username: String,
    balance: i32,
}

#[derive(Serialize)]
pub struct Stats {
    users: Vec<User>,
}

#[derive(Serialize)]
pub struct Pagination {
    offset: i32,
    limit: i32,
}

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
