use anyhow::{anyhow, Context};
use axum::http::header::SET_COOKIE;
use axum::http::HeaderMap;
use axum::response::Redirect;
use axum::{extract::State, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::error::AppError;
use crate::AppState;
use crate::database::user::User;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SignUpUserRequest {
    pub display_id: String,
    pub username: String,
    pub password: String,
}

pub async fn sign_up(
    State(app): State<AppState>,
    Json(req): Json<SignUpUserRequest>,
) -> anyhow::Result<impl IntoResponse, AppError> {
    let id = uuid::Uuid::new_v4();
    let user = User {
        id: id.into(),
        display_id: req.display_id,
        username: req.username,
    };

    app.db.create_user(&user).await?;
    app.db.save_password(user.display_id, req.password).await?;

    Ok(())
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SignInUserRequest {
    pub display_id: String,
    pub password: String,
}

pub async fn sign_in(
    State(app): State<AppState>,
    Json(req): Json<SignInUserRequest>,
) -> anyhow::Result<impl IntoResponse, AppError> {
    let user = app.db.get_user_by_display_id(&req.display_id).await?.ok_or(anyhow!("User does not exist"))?;

    if !app.db.verify_user_password(req.display_id, req.password).await? {
        return Err(anyhow!("Invalid password").into());
    }

    let cookie_value = app.db.create_session(user.display_id).await?;

    let mut headers = HeaderMap::new();

    headers.insert(
        SET_COOKIE,
        format!("session_id={cookie_value}; HttpOnly; SameSite=Strict").parse().with_context(|| "Failed to parse cookie")?
    );

    Ok((headers, Redirect::to("/me")))
}

pub async fn me(
    user: User
) -> anyhow::Result<impl IntoResponse, AppError> {
    Ok(Json(user))
}