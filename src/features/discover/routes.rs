use axum::{Router, routing::get};

use crate::{app::state::AppState, features::discover::handlers};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/discover/movie", get(handlers::discover_movies))
        .route("/discover/tv", get(handlers::discover_tv_series))
}
