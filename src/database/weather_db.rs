use crate::database::AppState;
use crate::errors::AppError;
use crate::models::weather::{WeatherDataOutput, WeatherDataInsert};

pub async fn insert_weather_data(
    state: &AppState,
    data: &WeatherDataInsert,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        INSERT INTO weather_data
        (city_id, temperature, humidity, pressure, weather_condition, raw_data)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
    )
        .bind(data.city_id)
        .bind(data.temperature)
        .bind(data.humidity as i32)  // Convert u8 to i32 for INTEGER column
        .bind(data.pressure as i32)  // Convert u16 to i32 for INTEGER column
        .bind(&data.weather_condition)
        .bind(&data.raw_data)
        .execute(&state.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(())
}

pub async fn get_all_weather_data(state: &AppState) -> Result<Vec<WeatherDataOutput>, AppError> {
    let data = sqlx::query_as::<_, WeatherDataOutput>
        ("SELECT DISTINCT ON (city_id) * FROM weather_data ORDER BY city_id, recorded_at DESC")
        .fetch_all(&state.pool)
        .await.map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(data)
}

pub async fn get_weather_by_specific_id(state: &AppState, city_id: i32) -> Result<Option<WeatherDataOutput>, AppError> {
    let data = sqlx::query_as::<_, WeatherDataOutput>(
        "SELECT * FROM weather_data WHERE city_id = $1 ORDER BY recorded_at DESC LIMIT 1",
    ).bind(city_id)
        .fetch_optional(&state.pool)
        .await.map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(data)
}