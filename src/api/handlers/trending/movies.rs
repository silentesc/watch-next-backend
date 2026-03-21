use axum::{
    Extension,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::{
    api::{
        db::models::Session, errors::AppError, handlers::trending::params::TrendingMoviesParams, services,
        tmdb::trending::models::TrendingMoviesResponse,
    },
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_trending_movies(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Path(time_window): Path<String>,
    Query(params): Query<TrendingMoviesParams>,
) -> Result<(StatusCode, TrendingMoviesResponse), AppError> {
    match services::trending::movies::get_trending_movies(app_state.client, time_window, params).await {
        Ok(response) => Ok((StatusCode::OK, response)),
        Err(app_error) => Err(app_error),
    }
}
