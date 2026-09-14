use std::time::Duration;

use sqlx::PgPool;

use crate::{
    debug,
    logger::enums::category::Category,
    persistence::table_utils::{cache, sessions},
    warn,
};

async fn cleanup(pool: &PgPool) {
    match sessions::clear_expired(pool).await {
        Ok(count) => debug!(Category::Worker, "Cleaned up {} expired sessions", count),
        Err(_) => warn!(Category::Worker, "Cleaning up expired sessions failed"),
    };

    match cache::clear_expired(pool).await {
        Ok(count) => debug!(Category::Worker, "Cleaned up {} expired cache entries", count),
        Err(_) => warn!(Category::Worker, "Cleaning up expired cache entries failed"),
    };
}

pub fn start_background_cleanup(pool: PgPool) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_mins(60));
        loop {
            interval.tick().await;
            cleanup(&pool).await;
        }
    });
}
