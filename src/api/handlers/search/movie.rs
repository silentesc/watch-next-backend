use axum::{Extension, extract::Query, http::StatusCode};

use crate::{
    api::{
        db::models::Session, errors::AppError, handlers::search::params::SearchMovieParams, services,
        tmdb::search::models::SearchMovieResponse,
    },
    state::AppState,
};

#[axum::debug_handler]
pub async fn search_movie(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Query(params): Query<SearchMovieParams>,
) -> Result<(StatusCode, SearchMovieResponse), AppError> {
    match services::search::movie::search_movie(app_state.client, params).await {
        Ok(response) => Ok((StatusCode::OK, response)),
        Err(app_error) => Err(app_error),
    }
}
