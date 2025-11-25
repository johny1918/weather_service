use crate::database::AppState;
use crate::errors::AppError;
use crate::models::cities::InputCityToDB;
use crate::database::city_db::{create_city, delete_city, get_city, update_city};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;

pub async fn insert_cities(
    State(state): State<AppState>,
    Json(params): Json<InputCityToDB>,
) -> Result<(StatusCode, String), AppError> {
    if params.name.is_empty()
        || params.latitude.is_nan()
        || params.longitude.is_nan()
        || params.country_code.is_empty()
    {
        return Err(AppError::NotFound);
    }
    let result = create_city(&state, &params).await.map_err(|e| {
        AppError::Validation(format!(
            "Failed to insert into db because of {}", e))
    })?;
    Ok((
        StatusCode::CREATED,
        format!("City {} added successfully with id {}", result.name, result.id),
    ))
}

pub async fn get_cities(State(state): State<AppState>, Path(id): Path<i32>)
    -> Result<(StatusCode, String), AppError> {
    if id.is_negative() {
        return Err(AppError::NotFound);
    }

    let result = get_city(&state, id.clone()).await.map_err(|e| {
        AppError::Validation(format!(
            "Failed to get city with id {} from db because of {}", id, e))
    }).or(Err(AppError::NotFound))?;

    Ok((StatusCode::OK, format!("City {} found successfully", result.name)))
}

pub async fn update_cities(State(state): State<AppState>, Path(id): Path<i32>, Json(params): Json<InputCityToDB> )
    -> Result<(StatusCode, String), AppError> {
    if id <= 0 {
        return Err(AppError::Validation("ID must be positive".to_string()));
    }
    if params.name.is_empty() || params.country_code.is_empty() {
        return Err(AppError::Validation("Name and country code cannot be empty".to_string()));
    }
    let result = update_city(&state, id, &params).await.map_err(|e| {
        AppError::Validation(format!(
            "Failed to update city because of {}", e))
    })?;
    Ok((
        StatusCode::OK,
        format!("City {} updated successfully with id {}", result.name, result.id),
    ))
}

pub async fn delete_cities(State(state): State<AppState>, Path(id): Path<i32>)
    -> Result<(StatusCode, String), AppError> {
    if id.is_negative() {
        return Err(AppError::NotFound);
    }
    let result = delete_city(&state, id.clone()).await.map_err(|e| {
        AppError::Validation(format!(
            "Failed to delete city with id {} from db because of {}", id, e))
    }).or(Err(AppError::NotFound))?;
    Ok((StatusCode::OK, format!("City {} deleted successfully", result.name)))
    }

