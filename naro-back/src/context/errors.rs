use axum::{http::StatusCode, response::IntoResponse, Json};

pub struct AppError {
    pub status: StatusCode,
    pub response: String,
}

impl<E> From<E> for AppError
where
    E: Into<String>,
{
    fn from(original_error: E) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            response: original_error.into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (self.status, Json(serde_json::json!({"error": self.response}))).into_response()
    }
}
