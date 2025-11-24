pub mod cities;
pub mod weather;

use crate::database::AppState;
use crate::errors::AppError;
use crate::models::cities::InputCityToDB;
use crate::models::cities::OutputCityFromDB;

pub async fn create_city(
    state: &AppState,
    params: &InputCityToDB,
) -> Result<OutputCityFromDB, AppError> {
    let add = sqlx::query_as::<_, OutputCityFromDB>(
        r#"
            INSERT INTO cities
            (name, country_code, latitude, longitude)
            VALUES ($1, $2, $3, $4)
            RETURNING id, name, country_code, latitude, longitude, created_at
        "#,
    )
    .bind(&params.name)
    .bind(&params.country_code)
    .bind(params.latitude)
    .bind(params.longitude)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(add)
}
