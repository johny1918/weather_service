use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize)]
pub struct InputCityToDB {
    pub name: String,
    pub country_code: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, FromRow, Serialize)]
pub struct OutputCityFromDB {
    pub id: i32,
    pub name: String,
    pub country_code: String,
    pub latitude: f64,
    pub longitude: f64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
