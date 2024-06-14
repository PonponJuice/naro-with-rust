use axum::{
    async_trait,
    extract::{FromRequestParts, Request, State},
    http::{request::Parts, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::{headers::Cookie, TypedHeader};

use crate::AppState;

use super::user::User;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, sqlx::FromRow, PartialEq, Eq)]
pub struct SessionId {
    pub session_id: String,
}

pub async fn auth_middleware(
    State(app): State<AppState>,
    TypedHeader(cookie): TypedHeader<Cookie>,
    mut req: Request,
    next: Next,
) -> anyhow::Result<impl IntoResponse, (StatusCode, &'static str)> {
    let session_id = cookie.get("session_id").ok_or((
        StatusCode::UNAUTHORIZED,
        "something wrong in getting session",
    ))?;

    let display_id = app
        .db
        .get_display_id_by_session_id(session_id)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get display ID",
            )
        })?
        .ok_or((StatusCode::UNAUTHORIZED, "please login"))?;

    let user = app
        .db
        .get_user_by_display_id(&display_id)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get user"))?
        .ok_or((StatusCode::UNAUTHORIZED, "please login"))?;

    let session_id = SessionId {
        session_id: session_id.to_string(),
    };

    req.extensions_mut().insert(user);
    req.extensions_mut().insert(app);
    req.extensions_mut().insert(session_id);

    Ok(next.run(req).await)
}

#[async_trait]
impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let user = parts
            .extensions
            .get::<Self>()
            .expect("User not found. Did you add auth_middleware?");
        Ok(user.clone())
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for SessionId
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let session_id = parts
            .extensions
            .get::<Self>()
            .expect("SessionId not found. Did you add auth_middleware?");
        Ok(session_id.clone())
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AppState
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let app = parts
            .extensions
            .get::<Self>()
            .expect("AppState not found. Did you add auth_middleware?");
        Ok(app.clone())
    }
}
