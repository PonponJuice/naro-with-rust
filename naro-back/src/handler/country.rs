use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::AppState;

pub async fn get_city_handler(
    State(app): State<AppState>,
    Path(cityname): Path<String>,
) -> anyhow::Result<impl IntoResponse, (StatusCode, &'static str)> {
    let city = app
        .db
        .get_city_by_id(cityname)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get city"))?
        .ok_or((StatusCode::NOT_FOUND, "City does not exist"))?;

    Ok(Json(city))
}
