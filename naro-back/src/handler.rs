use crate::database::auth;
use crate::AppState;
use axum::{
    middleware::from_fn_with_state,
    response::IntoResponse,
    routing::{get, post},
};

pub mod country;
pub mod session;

async fn root() -> impl IntoResponse {
    "Hello, World!"
}

async fn ping() -> impl IntoResponse {
    "pong"
}

pub fn make_router(state: AppState) -> axum::Router {
    let public = axum::Router::new()
        .route("/", get(root))
        .route("/ping", get(ping))
        .route("/signup", post(session::sign_up))
        .route("/login", post(session::login));

    let private = axum::Router::new()
        .route("/me", get(session::me))
        .route("/logout", post(session::logout))
        .route("/city/:cityname", get(country::get_city_handler))
        .route("/cities", post(country::post_city_handler))
        .route_layer(from_fn_with_state(state.clone(), auth::auth_middleware));

    axum::Router::new()
        .nest("/", public)
        .nest("/", private)
        .with_state(state)
}
