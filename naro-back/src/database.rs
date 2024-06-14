use anyhow::Context;
use async_sqlx_session::MySqlSessionStore;
use sqlx::mysql;

use crate::DataBase;

pub mod auth;
pub mod user;
pub mod user_password;
pub mod user_session;

pub fn get_options() -> anyhow::Result<mysql::MySqlConnectOptions> {
    let hostname = std::env::var("MYSQL_HOSTNAME").unwrap_or_else(|_| "localhost".to_string());
    let port = std::env::var("MYSQL_PORT")
        .unwrap_or_else(|_| "3306".to_string())
        .parse::<u16>()
        .with_context(|| "DB_PORT must be a number")?;
    let username = std::env::var("MYSQL_USERNAME").unwrap_or_else(|_| "root".to_string());
    let password = std::env::var("MYSQL_PASSWORD").unwrap_or_else(|_| "password".to_string());
    let database = std::env::var("MYSQL_DATABASE").unwrap_or_else(|_| "world".to_string());

    let options = mysql::MySqlConnectOptions::new()
        .host(&hostname)
        .port(port)
        .username(&username)
        .password(&password)
        .database(&database);

    Ok(options)
}

const MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

impl DataBase {
    pub async fn connect(options: mysql::MySqlConnectOptions) -> anyhow::Result<Self> {
        let pool = mysql::MySqlPool::connect_with(options).await?;
        let session_store =
            MySqlSessionStore::from_client(pool.clone()).with_table_name("sessions");
        Ok(DataBase {
            pool,
            session_store,
            bcrypt_cost: bcrypt::DEFAULT_COST,
        })
    }

    pub async fn init(&self) -> anyhow::Result<()> {
        MIGRATOR.run(&self.pool).await?;
        self.session_store.migrate().await?;
        Ok(())
    }
}
