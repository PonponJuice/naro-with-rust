use axum::http::header::SET_COOKIE;
use axum::http::{HeaderMap, StatusCode};
use axum::response::Redirect;
use axum::{extract::State, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::database::user::User;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SignUpUserRequest {
    pub display_id: String,
    pub username: String,
    pub password: String,
}

fn is_valid_password(password: &str) -> bool {
    password.len() >= 8
        && password.chars().any(|c| c.is_ascii_uppercase())
        && password.chars().any(|c| c.is_ascii_lowercase())
        && password.chars().any(|c| c.is_numeric())
}

pub async fn sign_up(
    State(app): State<AppState>,
    Json(req): Json<SignUpUserRequest>,
) -> anyhow::Result<impl IntoResponse, (StatusCode , &'static str)> {
    if req.display_id.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Display ID is empty"));
    }
    if req.username.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Username is empty"));
    }
    if is_valid_password(&req.password) {
        return Err((StatusCode::BAD_REQUEST, "Password is invalid"));
    }
    if app.db
        .get_user_by_display_id(&req.display_id)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get user"))?.is_some() {
            return Err((StatusCode::BAD_REQUEST, "User already exists"));
        }

    let id = uuid::Uuid::new_v4();
    let user = User {
        id: id.into(),
        display_id: req.display_id,
        username: req.username,
    };

    app.db
        .create_user(&user)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create user"))?;
    app.db
        .save_password(user.display_id, req.password)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to save password"))?;

    Ok(())
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SignInUserRequest {
    pub display_id: String,
    pub password: String,
}

pub async fn login(
    State(app): State<AppState>,
    Json(req): Json<SignInUserRequest>,
) -> anyhow::Result<impl IntoResponse, (StatusCode , &'static str)> {
    let user = app.db
        .get_user_by_display_id(&req.display_id)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get user"))?
        .ok_or((StatusCode::UNAUTHORIZED, "User does not exist"))?;

    if !app.db
        .verify_user_password(req.display_id, req.password)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to verify password"))? {
            return Err((StatusCode::UNAUTHORIZED, "Password is incorrect"));
        }

    let cookie_value = app.db
        .create_session(user.display_id)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create session"))?;

    let mut headers = HeaderMap::new();

    headers.insert(
        SET_COOKIE,
        format!("session_id={cookie_value}; HttpOnly; SameSite=Strict")
            .parse()
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create cookie"))?
    );

    Ok((headers, Redirect::to("/me")))
}

pub async fn logout(
    State(app): State<AppState>,
    session_id: crate::database::auth::SessionId
) -> anyhow::Result<impl IntoResponse, (StatusCode , &'static str)> {
    app.db
        .delete_session(session_id.session_id)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete session"))?;
    Ok(Redirect::to("/"))
}

pub async fn me(
    user: User
) -> anyhow::Result<impl IntoResponse, (StatusCode , &'static str)> {
    Ok(Json(user))
}