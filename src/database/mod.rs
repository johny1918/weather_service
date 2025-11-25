pub mod city_db;
pub mod weather_db;

use crate::errors::AppError;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

#[allow(dead_code)]
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

pub async fn connect_to_db() -> Result<PgPool, AppError> {
    dotenvy::dotenv().ok();
    //Get string from environment variable
    let connection_path =
        std::env::var("DATABASE_URL").map_err(|e| AppError::Validation(e.to_string()))?;

    //Create pool
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(&connection_path)
        .await
        .map_err(|e| AppError::DatabaseConnectionError(e.to_string()))
}
