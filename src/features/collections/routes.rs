use axum::{Router, routing::get};

use crate::{app::state::AppState, features::collections::handlers};

pub fn router() -> Router<AppState> {
    Router::new().route("/collection/{collection_id}", get(handlers::get_collection_details))
}
