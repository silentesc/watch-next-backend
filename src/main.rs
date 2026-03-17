use std::env;

use crate::{logger::enums::category::Category, setup::setup_tcp_listener};

mod api;
mod logger;
mod setup;
mod state;

#[tokio::main]
async fn main() {
    setup::load_env();

    setup::setup_logging();

    let pool = setup::connect_postgres().await;
    setup::delete_tables(&pool).await;
    setup::check_create_tables(&pool).await;

    let app_state = setup::setup_app_state(pool);

    let router = setup::setup_router(app_state);

    let addr = env::var("SERVE_ADDR").expect("SERVE_ADDR env variable should be set by dotenv");
    let listener = setup_tcp_listener(&addr).await;

    info!(Category::Setup, "Listening on {}", addr);
    setup::serve(listener, router).await;

    info!(Category::Setup, "Shutdown");
}
