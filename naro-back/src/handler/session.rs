use axum::http::header::SET_COOKIE;
use axum::http::{HeaderMap, StatusCode};
use axum::response::Redirect;
use axum::{extract::State, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::context::errors::AppError;
use crate::context::validated_json::ValidatedJson;
use crate::database::auth::MyUuid;
use crate::database::user::User;
use crate::AppState;

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct SignUpUserRequest {
    #[validate(length(min = 1, message = "Display ID is empty"))]
    pub display_id: String,
    #[validate(length(min = 1, message = "Username is empty"))]
    pub username: String,
    #[validate(custom(function = "is_valid_password", message = "Password is invalid"))]
    pub password: String,
}

fn is_valid_password(password: &str) -> Result<(), ValidationError> {
    if password.len() >= 8
        && password.chars().any(|c| c.is_ascii_uppercase())
        && password.chars().any(|c| c.is_ascii_lowercase())
        && password.chars().any(|c| c.is_numeric())
    {
        Ok(())
    } else {
        Err(ValidationError::new(""))
    }
}

pub async fn sign_up(
    State(app): State<AppState>,
    ValidatedJson(req): ValidatedJson<SignUpUserRequest>,
) -> anyhow::Result<impl IntoResponse, AppError> {
    if app
        .db
        .get_user_by_display_id(&req.display_id)
        .await
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Failed to get user"))?
        .is_some()
    {
        return Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "Display ID is already used",
        ));
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
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Failed to create user"))?;
    app.db
        .save_password(user.display_id, req.password)
        .await
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Failed to save password"))?;

    Ok(())
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
pub struct SignInUserRequest {
    #[validate(length(min = 1, message = "Display ID is empty"))]
    pub display_id: String,
    #[validate(length(min = 1, message = "Password is empty"))]
    pub password: String,
}

pub async fn login(
    State(app): State<AppState>,
    ValidatedJson(req): ValidatedJson<SignInUserRequest>,
) -> anyhow::Result<impl IntoResponse, AppError> {
    let user = app
        .db
        .get_user_by_display_id(&req.display_id)
        .await
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Failed to get user"))?
        .ok_or(AppError::new(
            StatusCode::UNAUTHORIZED,
            "User does not exist",
        ))?;

    if !app
        .db
        .verify_user_password(req.display_id, req.password)
        .await
        .map_err(|_| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to verify password",
            )
        })?
    {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "Password is incorrect",
        ));
    }

    let cookie_value = app.db.create_session(user.id).await.map_err(|_| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to create session",
        )
    })?;

    let mut headers = HeaderMap::new();

    headers.insert(
        SET_COOKIE,
        format!("session_id={cookie_value}; HttpOnly; SameSite=Strict")
            .parse()
            .map_err(|_| {
                AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Failed to create cookie")
            })?,
    );

    Ok((headers, Redirect::to("/me")))
}

pub async fn logout(
    State(app): State<AppState>,
    session_id: crate::database::auth::SessionId,
) -> anyhow::Result<impl IntoResponse, AppError> {
    app.db
        .delete_session(session_id.session_id)
        .await
        .map_err(|_| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to delete session",
            )
        })?;

    Ok(Redirect::to("/"))
}

pub async fn me(
    uuid: MyUuid,
    State(app): State<AppState>,
) -> anyhow::Result<impl IntoResponse, AppError> {
    let user = app
        .db
        .get_user_by_uuid(&uuid.uuid)
        .await
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Failed to get user"))?
        .ok_or(AppError::new(
            StatusCode::UNAUTHORIZED,
            "User does not exist",
        ))?;

    Ok(Json(user))
}
