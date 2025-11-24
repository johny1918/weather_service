use tracing::info;
use crate::errors::AppError;

mod database;
mod errors;
mod logic;
mod models;
mod routes;

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    info!("Tracing initialized");
}

#[tokio::main]
async fn main() -> Result<(), AppError>{
    init_tracing();
    Ok(())
}
