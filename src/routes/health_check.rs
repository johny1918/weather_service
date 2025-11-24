use axum::Json;

pub async fn health_check() -> Json<String> {
    "status: ok".to_string().into()
}
