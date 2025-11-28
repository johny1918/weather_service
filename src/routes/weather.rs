
use axum::extract::{Path, State};
use reqwest::StatusCode;

use crate::database::AppState;
use crate::database::weather_db::{get_all_weather_data, get_weather_by_specific_id};
use crate::errors::AppError;

pub async fn get_weather(State(state): State<AppState>)
                         -> Result<(StatusCode, String), AppError> {
        let data = get_all_weather_data(&state)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok((StatusCode::OK,
        format!("Data retrieved successfully {}", data
            .into_iter()
            .map(|d| format!("{:?}", d))
            .collect::<Vec<String>>()
            .join(", "))))
}

pub async fn get_weather_by_id(State(state): State<AppState>, Path(city_id): Path<i32>)
    -> Result<(StatusCode, String), AppError> {

    if city_id.is_negative() {
        return Err(AppError::Validation("City ID must be a positive integer".to_string()));
    }
    let data = get_weather_by_specific_id(&state, city_id)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok((StatusCode::OK, format!("Data retrieved successfully: {:?}", data)))
}