use axum::{Extension, extract::Query, http::StatusCode};

use crate::{
    api::{
        db::models::Session, errors::AppError, handlers::search::params::SearchCollectionParams, services,
        tmdb::search::models::SearchCollectionResponse,
    },
    state::AppState,
};

#[axum::debug_handler]
pub async fn search_collection(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Query(params): Query<SearchCollectionParams>,
) -> Result<(StatusCode, SearchCollectionResponse), AppError> {
    match services::search::collection::search_collection(app_state.client, params).await {
        Ok(response) => Ok((StatusCode::OK, response)),
        Err(app_error) => Err(app_error),
    }
}
