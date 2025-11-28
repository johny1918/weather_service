mod city;
mod health_check;
mod weather;

use crate::database::AppState;
use crate::routes::city::{insert_cities, delete_cities, update_cities, get_cities};
use crate::routes::health_check::health_check;
use axum::{
    Router,
    routing::{get, post},
};
use axum::body::Body;
use std::time::Duration;
use tower_http::trace::TraceLayer;
use tracing::Span;
use crate::routes::weather::{get_weather, get_weather_by_id};

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
        .route("/add-city", post(insert_cities))
        .route("/cities/{id}", get(get_cities))
        .route("/update-city/{id}", post(update_cities))
        .route("/delete-city/{id}", get(delete_cities))
        .route("/weather", get(get_weather))
        .route("/weather/{city_id}", get(get_weather_by_id))
        .route("/health", get(health_check))
        .layer(trace_layer)
        .with_state(state)
}
