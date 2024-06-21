use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::{context::errors::AppError, database::country::City, AppState};

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
            response: Json(serde_json::json!("Failed to get city")),
        })?
        .ok_or(AppError {
            status: StatusCode::NOT_FOUND,
            response: Json(serde_json::json!("city does not exist")),
        })?;

    Ok(Json(city))
}

pub async fn post_city_handler(
    State(app): State<AppState>,
    Json(city): Json<City>,
) -> anyhow::Result<impl IntoResponse, AppError> {
    let city = app.db.create_city(city).await.map_err(|_| AppError {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        response: Json(serde_json::json!("Failed to create city")),
    })?;

    Ok(Json(city))
}
