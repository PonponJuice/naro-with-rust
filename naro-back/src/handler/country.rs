use crate::{context::errors::AppError, database::country::City, AppState};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

pub async fn get_city_handler(
    State(app): State<AppState>,
    Path(cityname): Path<String>,
) -> anyhow::Result<impl IntoResponse, AppError> {
    let city = app
        .db
        .get_city_by_id(cityname)
        .await
        .map_err(|_| AppError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            response: "Failed to get city".to_string(),
        })?
        .ok_or(AppError {
            status: StatusCode::NOT_FOUND,
            response: "city does not exist".to_string(),
        })?;

    Ok(Json(city))
}

pub async fn post_city_handler(
    State(app): State<AppState>,
    Json(city): Json<City>,
) -> anyhow::Result<impl IntoResponse, AppError> {
    let city = app.db.create_city(city).await.map_err(|_| AppError {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        response: "Failed to create city".to_string(),
    })?;

    Ok(Json(city))
}
