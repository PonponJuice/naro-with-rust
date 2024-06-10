use axum::{middleware::from_fn_with_state, response::IntoResponse, routing::{get, post}};
use crate::database::auth;
use crate::AppState;

pub mod session;

async fn root() -> impl IntoResponse {
    "Hello, World!"
}

pub fn make_router(state: AppState) -> axum::Router {
    let public = axum::Router::new()
        .route("/", get(root))
        .route("/signup", post(session::sign_up))
        .route("/signin", post(session::sign_in));
    
    let private = axum::Router::new()
        .route("/me", get(session::me))
        .route_layer(from_fn_with_state(state.clone(), auth::auth_middleware));
    
    axum::Router::new()
        .nest("/", public)
        .nest("/", private)
        .with_state(state)
}