use axum::Router;
use clicker::*;
use gamble::*;
use health::*;
use stats::*;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::state::State;

pub mod clicker;
pub mod gamble;
pub mod health;
pub mod stats;

#[derive(OpenApi)]
#[openapi(
    paths(check_health, slots, roulette, stats, click),
    components(schemas(
        Health,
        GambleResult,
        Stats,
        User,
        ClickResponse,
        UserId,
        Slot,
        Roulette
    )),
    info(
        title = "Evil Lumios API",
        description = "API for gambling and health check"
    )
)]
struct ApiDoc;

pub async fn start(state: State) {
    let app = Router::new()
        .route("/", axum::routing::get(check_health))
        .route("/slots", axum::routing::get(slots))
        .route("/routette", axum::routing::get(roulette))
        .route("/stats", axum::routing::get(stats))
        .route("/clicker", axum::routing::get(click))
        .with_state(state)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
