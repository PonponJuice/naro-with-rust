use axum::{
    async_trait,
    extract::{FromRequestParts, Request, State},
    http::{request::Parts, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use axum_extra::{headers::Cookie, TypedHeader};

use crate::AppState;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, sqlx::FromRow, PartialEq, Eq)]
pub struct SessionId {
    pub session_id: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, sqlx::FromRow, PartialEq, Eq)]
pub struct MyUuid{
    pub uuid: String,
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

    let uuid = app
        .db
        .get_uuid_by_session_id(session_id)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get display ID",
            )
        })?
        .ok_or((StatusCode::UNAUTHORIZED, "please login"))?;

    
    let session_id = SessionId {
        session_id: session_id.to_string(),
    };
    let uuid = MyUuid {
        uuid: uuid.parse().unwrap(),
    };

    req.extensions_mut().insert(uuid);
    req.extensions_mut().insert(session_id);

    Ok(next.run(req).await)
}

#[async_trait]
impl<S> FromRequestParts<S> for MyUuid
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let uuid = parts
            .extensions
            .get::<Self>()
            .expect("MyUuid not found. Did you add auth_middleware?");
        Ok(uuid.clone())
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
