use std::{env, time::Duration};

use axum::{
    Router,
    http::{HeaderMap, HeaderValue, Method, header},
    middleware::from_fn_with_state,
    routing::{get, post},
};
use axum_extra::extract::cookie::Key;
use dotenv::dotenv;
use reqwest::Client;
use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

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
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(60))
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
    let tmdb_api_key = env::var("TMDB_API_KEY").expect("TMDB_API_KEY env variable should be set by dotenv");
    let auth_header_value = HeaderValue::from_str(format!("Bearer {}", tmdb_api_key).as_str())
        .expect("TMDB api key should be converted to HeaderValue");

    let mut headers = HeaderMap::new();
    headers.append("Authorization", auth_header_value);
    headers.append("accept", HeaderValue::from_static("application/json"));

    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .connect_timeout(Duration::from_secs(10))
        .default_headers(headers)
        .build()
        .expect("Reqwest client should be built");

    let key = Key::generate();
    AppState { pool, client, key }
}

pub fn setup_router(app_state: AppState) -> Router {
    let origins = env::var("CORS_ALLOWED_ORIGINS").expect("CORS_ALLOWED_ORIGINS env variable should be set by dotenv");
    let origins: Vec<HeaderValue> = origins
        .split(",")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().map_err(|_| format!("Invalid origin: {}", s)))
        .collect::<Result<Vec<_>, _>>()
        .expect("One or more origins were invalid");

    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .allow_credentials(true);

    let protected_routes = Router::new()
        .route("/me", get(handlers::me::handler::me))
        .route("/discover/movie", get(handlers::discover::movie::discover))
        .route("/genre/movie/list", get(handlers::genres::movie::get_movie_genres))
        .route(
            "/configuration/languages",
            get(handlers::configuration::languages::get_languages),
        )
        .route("/movie/{movie_id}", get(handlers::movies::details::get_movie_details))
        .layer(from_fn_with_state(
            app_state.clone(),
            middleware::auth::validate_session,
        ));

    Router::new()
        .route("/", get(handlers::root::handler::root))
        .route("/auth/register", post(handlers::auth::handler::register))
        .route("/auth/login", post(handlers::auth::handler::login))
        .route("/auth/logout", post(handlers::auth::handler::logout))
        .merge(protected_routes)
        .with_state(app_state)
        .layer(cors)
}

pub async fn setup_tcp_listener(addr: &str) -> TcpListener {
    TcpListener::bind(addr)
        .await
        .unwrap_or_else(|err| panic!("Listener should bind to {}: {}", addr, err))
}

pub async fn serve(listener: TcpListener, router: Router) {
    axum::serve(listener, router)
        .await
        .unwrap_or_else(|err| panic!("App should be served: {:#?}", err));
}
