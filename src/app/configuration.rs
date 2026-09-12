use std::{
    env::{self, VarError},
    time::Duration,
};

use axum_extra::extract::cookie::Key;
use dotenv::dotenv;
use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::{
    app::state::AppState,
    debug,
    integrations::tmdb::client::TmdbClient,
    logger::{
        Logger,
        enums::{category::Category, log_level::LogLevel},
    },
};

pub fn load_env() {
    dotenv().ok();
}

pub fn setup_logging() {
    let log_level_env = env::var("LOG_LEVEL").expect("LOG_LEVEL env variable should be set by dotenv");
    let log_level = LogLevel::from_string(log_level_env.as_str()).expect("Log level env variable should be valid");
    Logger::set_log_level(&log_level);
    debug!(
        Category::Setup,
        "Logging has been setup with log level {}",
        &log_level.to_string()
    );
}

pub async fn connect_postgres() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL env variable should be set by dotenv");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(60))
        .connect(&database_url)
        .await
        .expect("Postgres should connect successfully");
    debug!(Category::Setup, "Connected to postgres database successfully");
    pool
}

pub async fn check_create_tables(pool: &PgPool) {
    sqlx::raw_sql(include_str!("../persistence/create_tables.sql"))
        .execute(pool)
        .await
        .expect("persistence/create_tables.sql should be executed");
    debug!(Category::Setup, "Performed table creation check");
}

pub fn setup_app_state(pool: PgPool) -> AppState {
    let tmdb_api_key = env::var("TMDB_API_KEY").expect("TMDB_API_KEY env variable should be set by dotenv");

    let tmdb_client = TmdbClient::new(tmdb_api_key).expect("Reqwest client should be built");

    let key = match env::var("COOKIE_KEY") {
        Ok(secret) => Key::from(secret.as_bytes()),
        Err(err) => {
            if err != VarError::NotPresent {
                panic!("COOKIE_KEY is set but something went wrong: {:#?}", err);
            }
            Key::generate()
        }
    };

    AppState { pool, tmdb_client, key }
}
