use axum::{
    Extension,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::{
    api::{
        db::models::Session, errors::AppError, handlers::movies::params::MovieRecommendationsParams, services,
        tmdb::movies::models::MovieRecommendationsResponse,
    },
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_movie_recommendations(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Path(movie_id): Path<i32>,
    Query(params): Query<MovieRecommendationsParams>,
) -> Result<(StatusCode, MovieRecommendationsResponse), AppError> {
    match services::movies::recommendations::get_movie_recommendations(app_state.client, movie_id, params).await {
        Ok(response) => Ok((StatusCode::OK, response)),
        Err(app_error) => Err(app_error),
    }
}
