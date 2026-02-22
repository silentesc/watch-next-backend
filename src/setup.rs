use std::env;

use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{get, post},
};
use axum_extra::extract::cookie::Key;
use dotenv::dotenv;
use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::net::TcpListener;

use crate::{
    api::{handlers, middleware},
    debug,
    logger::{
        Logger,
        enums::{category::Category, log_level::LogLevel},
    },
    state::AppState,
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
    let pool: PgPool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Postgres should connect successfully");
    debug!(Category::Setup, "Connected to postgres database successfully");

    pool
}

pub async fn check_create_tables(pool: &PgPool) {
    sqlx::raw_sql(include_str!("query/create_tables.sql"))
        .execute(pool)
        .await
        .expect("query/create_tables.sql should be executed");
    debug!(Category::Setup, "Performed table creation check");
}

pub async fn delete_tables(pool: &PgPool) {
    sqlx::raw_sql(include_str!("query/delete_tables.sql"))
        .execute(pool)
        .await
        .expect("query/delete_tables.sql should be executed");
    debug!(Category::Setup, "Deleted all tables");
}

pub fn setup_app_state(pool: PgPool) -> AppState {
    let key = Key::generate();
    AppState { pool, key }
}

pub fn setup_router(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(handlers::root::root))
        .route("/auth/register", post(handlers::auth::register))
        .route("/auth/login", post(handlers::auth::login))
        .route(
            "/protected",
            get(handlers::protected::protected).layer(from_fn_with_state(
                app_state.clone(),
                middleware::auth::validate_session,
            )),
        )
        .with_state(app_state)
}

pub async fn setup_tcp_listener(addr: &str) -> TcpListener {
    TcpListener::bind(addr)
        .await
        .unwrap_or_else(|err| panic!("Listener should bind to {}: {}", addr, err))
}

pub async fn serve(listener: TcpListener, router: Router) {
    axum::serve(listener, router)
        .await
        .unwrap_or_else(|err| panic!("App should be served: {:#}", err));
}
