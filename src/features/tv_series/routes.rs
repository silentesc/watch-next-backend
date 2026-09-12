use axum::{Router, routing::get};

use crate::{app::state::AppState, features::tv_series::handlers};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/tv/{series_id}", get(handlers::get_series_details))
        .route("/tv/{series_id}/videos", get(handlers::get_series_videos))
        .route(
            "/tv/{series_id}/recommendations",
            get(handlers::get_series_recommendations),
        )
        .route("/tv/{series_id}/similar", get(handlers::get_similar_series))
}
