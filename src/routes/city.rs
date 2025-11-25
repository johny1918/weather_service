use crate::database::AppState;
use crate::errors::AppError;
use crate::models::cities::InputCityToDB;
use crate::database::city_db::{create_city, get_cities};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;

pub async fn insert_city(
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

pub async fn get_city(State(state): State<AppState>, Path(id): Path<i32>)
    -> Result<(StatusCode, String), AppError> {
    if id.is_negative() {
        return Err(AppError::NotFound);
    }

    let result = get_cities(&state, id.clone()).await.map_err(|e| {
        AppError::Validation(format!(
            "Failed to get city with id {} from db because of {}", id, e))
    }).or(Err(AppError::NotFound))?;

    Ok((StatusCode::OK, format!("City {} found successfully", result.name)))
}
