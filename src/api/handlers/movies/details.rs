use axum::{
    Extension,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::{
    api::{
        db::models::Session, errors::AppError, handlers::movies::params::MovieDetailsParams, services,
        tmdb::models::MovieDetails,
    },
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_movie_details(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Path(movie_id): Path<i32>,
    Query(params): Query<MovieDetailsParams>,
) -> Result<(StatusCode, MovieDetails), AppError> {
    match services::movies::details::get_movie_details(app_state.client, movie_id, params).await {
        Ok(response) => Ok((StatusCode::OK, response)),
        Err(app_error) => Err(app_error),
    }
}
