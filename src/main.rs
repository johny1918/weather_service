use crate::errors::AppError;
use crate::routes::create_routing;
use database::connect_to_db;
use tracing::{error, info};
use crate::models::weather::WeatherApiClient;
use crate::scheduler::WeatherScheduler;

mod database;
mod errors;
mod scheduler;
mod models;
mod routes;

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    info!("Tracing initialized");
}

async fn start_server() -> Result<(), AppError> {
    //Initialize the database connection pool
    let db_pool = connect_to_db().await?;
    info!("Database connection pool initialized");
    let db_pool = database::AppState::new(db_pool);
    let app = create_routing(db_pool.clone());

    let api_key = std::env::var("API_KEY")
        .map_err(|_| AppError::Validation("API_KEY environment variable not set".to_string()))?;
    let weather_client = WeatherApiClient::new(api_key);
    let scheduler = WeatherScheduler::new(db_pool, weather_client)
        .await
        .map_err(|e| AppError::Validation(e.to_string()))?;

    tokio::spawn(
        async move {
            if let Err(e) = scheduler.start().await {
                error!("Scheduler error: {}", e);
            }
        }
    );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .map_err(|e| AppError::Validation(e.to_string()))?;
    info!("Server listening on port 3000");

    axum::serve(listener, app.into_make_service())
        .await
        .map_err(|e| AppError::Validation(e.to_string()))?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    //Initialize tracing
    init_tracing();

    //Start the server
    start_server().await?;

    Ok(())
}
