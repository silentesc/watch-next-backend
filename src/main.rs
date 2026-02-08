use std::env;

use axum::{
    Router,
    routing::{get, post},
};
use dotenv::dotenv;
use tokio::net::TcpListener;

use crate::{
    api::handlers::{auth::AuthHandler, root::RootHandler},
    logger::{
        Logger,
        enums::{category::Category, log_level::LogLevel},
    },
};

mod api;
mod logger;

#[tokio::main]
async fn main() {
    // Load .env variables
    dotenv().ok();

    // Setup logging
    let log_level_env = env::var("LOG_LEVEL").expect("Env variable should be set by dotenv");
    let log_level = LogLevel::from_string(log_level_env.as_str()).expect("Log level env variable should be valid");
    Logger::set_log_level(&log_level);
    debug!(Category::Setup, "Setup logging with log level {}", &log_level.to_string());

    // Setup routes
    let app = Router::new()
        // Default ping, just says hello
        .route("/", get(RootHandler::root))
        // Registering
        .route("/register", post(AuthHandler::register))
        // Login
        .route("/login", post(AuthHandler::login));

    // Setup TcpListener
    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr).await.unwrap_or_else(|_| panic!("Listener should bind to {}", addr));

    // Serve app via listener
    info!(Category::Setup, "Listening on {}", addr);
    axum::serve(listener, app).await.ok();

    info!(Category::Setup, "Shutdown");
}
