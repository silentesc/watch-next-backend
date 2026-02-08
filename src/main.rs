use std::env;

use dotenv::dotenv;

use crate::logger::{
    enums::{category::Category, log_level::LogLevel},
    logger::Logger,
};

mod logger;

fn main() {
    dotenv().ok();

    let log_level_env = env::var("LOG_LEVEL").expect("Env variable should be set by dotenv");
    let log_level = LogLevel::from_string(log_level_env.as_str()).expect("Log level env variable should be valid");
    Logger::set_log_level(log_level);

    trace!(Category::Setup, "Hello Trace");
    debug!(Category::Setup, "Hello Debug");
    info!(Category::Setup, "Hello Info");
    warn!(Category::Setup, "Hello Warn");
    error!(Category::Setup, "Hello Error");
}
