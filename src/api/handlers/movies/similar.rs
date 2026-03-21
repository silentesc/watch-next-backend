use axum::{
    Extension,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::{
    api::{
        db::models::Session, errors::AppError, handlers::movies::params::SimilarMoviesParams, services,
        tmdb::movies::models::SimilarMoviesResponse,
    },
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_similar_movies(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Path(movie_id): Path<i32>,
    Query(params): Query<SimilarMoviesParams>,
) -> Result<(StatusCode, SimilarMoviesResponse), AppError> {
    match services::movies::similar::get_similar_movies(app_state.client, movie_id, params).await {
        Ok(response) => Ok((StatusCode::OK, response)),
        Err(app_error) => Err(app_error),
    }
}
