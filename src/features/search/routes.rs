use axum::{Router, routing::get};

use crate::{app::state::AppState, features::search::handlers};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/search/movie", get(handlers::search_movie))
        .route("/search/tv", get(handlers::search_series))
        .route("/search/collection", get(handlers::search_collection))
}
