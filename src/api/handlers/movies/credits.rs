use axum::{
    Extension,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::{
    api::{
        db::models::Session, errors::AppError, handlers::movies::params::MovieCreditsParams, services,
        tmdb::movies::models::MovieCreditsResponse,
    },
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_movie_credits(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Path(movie_id): Path<i32>,
    Query(params): Query<MovieCreditsParams>,
) -> Result<(StatusCode, MovieCreditsResponse), AppError> {
    match services::movies::credits::get_movie_credits(app_state.client, movie_id, params).await {
        Ok(response) => Ok((StatusCode::OK, response)),
        Err(app_error) => Err(app_error),
    }
}
