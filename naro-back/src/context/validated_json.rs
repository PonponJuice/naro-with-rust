use super::errors::AppError;
use axum::{
    async_trait,
    extract::{FromRequest, Request},
    http::StatusCode,
    Json,
};
use serde::de::DeserializeOwned;
use validator::Validate;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|_| AppError {
                status: StatusCode::BAD_REQUEST,
                response: Json(serde_json::json!("Bad Request")),
            })?;

        value.validate().map_err(|e| AppError {
            status: StatusCode::BAD_REQUEST,
            response: Json(serde_json::json!(e)),
        })?;
        Ok(ValidatedJson(value))
    }
}
