use axum::Router;
use clicker::*;
use gamble::*;
use health::*;
use stats::*;

use crate::state::State;

pub mod clicker;
pub mod gamble;
pub mod health;
pub mod stats;

pub async fn start(state: State) {
    tracing::info!("Starting API server");
    let app = Router::new()
        .route("/", axum::routing::get(check_health))
        .route("/slots", axum::routing::get(slots))
        .route("/routette", axum::routing::get(roulette))
        .route("/stats", axum::routing::get(stats))
        .route("/clicker", axum::routing::get(click))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
