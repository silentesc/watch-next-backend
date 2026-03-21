use axum::{
    Extension,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::{
    api::{
        db::models::Session, errors::AppError, handlers::movies::params::MovieVideosParams, services,
        tmdb::movies::models::MovieVideosResponse,
    },
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_movie_videos(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Path(movie_id): Path<i32>,
    Query(params): Query<MovieVideosParams>,
) -> Result<(StatusCode, MovieVideosResponse), AppError> {
    match services::movies::videos::get_movie_videos(app_state.client, movie_id, params).await {
        Ok(response) => Ok((StatusCode::OK, response)),
        Err(app_error) => Err(app_error),
    }
}
