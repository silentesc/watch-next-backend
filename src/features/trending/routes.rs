use axum::{Router, routing::get};

use crate::{app::state::AppState, features::trending::handlers};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/trending/movie/{time_window}", get(handlers::get_trending_movies))
        .route("/trending/tv/{time_window}", get(handlers::get_trending_series))
}
