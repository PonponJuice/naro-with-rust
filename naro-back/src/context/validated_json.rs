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
            .map_err(|_| AppError::new(StatusCode::BAD_REQUEST, "Bad Request"))?;

        value.validate().map_err(|e| {
            let mut message = "".to_string();
            let errors = e.field_errors();
            'out: for (_, v) in errors.into_iter() {
                for validation_error in v {
                    if let Some(msg) = validation_error.clone().message {
                        message.push_str(&msg);
                        break 'out;
                    }
                }
            }

            AppError::new(StatusCode::BAD_REQUEST, message)
        })?;
        Ok(ValidatedJson(value))
    }
}
