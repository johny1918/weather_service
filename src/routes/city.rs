use crate::database::AppState;
use crate::errors::AppError;
use crate::models::cities::InputCityToDB;
use crate::models::create_city;
use axum::extract::State;
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
    create_city(&state, &params).await.map_err(|e| {
        AppError::Validation(format!(
            "Failed to insert into db because of {}", e))
    })?;
    Ok((
        StatusCode::CREATED,
        format!("City {} added successfully", params.name),
    ))
}
