use crate::database::AppState;
use axum::{Json, Router, routing::get};
use tower_http::trace::TraceLayer;

pub fn create_routing(state: AppState) -> Router {
    Router::new()
        .route("/", get(health_check))
        .layer(TraceLayer::new_for_http())
        .with_state(state)

}

async fn health_check() -> Json<String> {
    "status: ok".to_string().into()
}
