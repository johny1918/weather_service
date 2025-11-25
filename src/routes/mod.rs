mod city;
mod health_check;

use crate::database::AppState;
use crate::routes::city::insert_city;
use crate::routes::health_check::health_check;
use axum::{
    Router,
    routing::{get, post},
};
use axum::body::Body;
use std::time::Duration;
use tower_http::trace::TraceLayer;
use tracing::Span;

pub fn create_routing(state: AppState) -> Router {

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(|request: &axum::http::Request<Body>| {
            tracing::info_span!(
                "request",
                method = %request.method().as_str(),
                uri = %request.uri(),
                version = ?request.version(),
            )
        })
        .on_request(|_request: &axum::http::Request<Body>, _span: &Span| {
            tracing::info!("started processing request")
        })
        .on_response(|response: &axum::http::Response<Body>, latency: Duration, _span: &Span| {
            let status = response.status();

            // Use match instead of variable for the level
            match status.as_u16() {
                200..=399 => {
                    tracing::info!(
                        status = %status,
                        latency = ?latency,
                        "request completed successfully"
                    )
                }
                400..=499 => {
                    tracing::warn!(
                        status = %status,
                        latency = ?latency,
                        "client error"
                    )
                }
                _ => {
                    tracing::error!(
                        status = %status,
                        latency = ?latency,
                        "server error"
                    )
                }
            }
        });

    Router::new()
        .route("/add-city", post(insert_city))
        .route("/cities/{id}", get(city::get_city))
        .route("/health", get(health_check))
        .layer(trace_layer)
        .with_state(state)
}
