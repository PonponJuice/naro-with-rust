use async_sqlx_session::MySqlSessionStore;
use dotenvy::dotenv;
use sqlx::{MySql, Pool};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod database;
mod handler;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().expect(".env file not found");

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axum_sandbox=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let options = database::get_options()?;
    let db = DataBase::connect(options).await?;
    db.init().await?;
    let state = AppState::new(db);
    let app = handler::make_router(state).layer(TraceLayer::new_for_http());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;
    Ok(())
}

#[derive(Clone)]
pub struct AppState {
    db: DataBase,
}

impl AppState {
    pub fn new(db: DataBase) -> Self {
        AppState { db }
    }
}

#[derive(Clone, Debug)]
pub struct DataBase {
    pool: Pool<MySql>,
    session_store: MySqlSessionStore,
    bcrypt_cost: u32,
}
