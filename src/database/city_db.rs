use crate::database::AppState;
use crate::errors::AppError;
use crate::models::cities::{InputCityToDB, OutputCityFromDB};

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

pub async fn get_cities(state: &AppState, id: i32)
                        -> Result<OutputCityFromDB, AppError> {
    let get = sqlx::query_as::<_, OutputCityFromDB>(
        r#"SELECT * FROM cities WHERE id = $1"#
    )   .bind(id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?
        .ok_or(AppError::Validation("Other kinda of error from trying to get a city".to_string()))?;

    Ok(get)
}