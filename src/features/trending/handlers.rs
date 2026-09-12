use crate::{
    app::{errors::AppError, state::AppState},
    features::trending::{
        dto::{TrendingMoviesParams, TrendingMoviesResponse, TrendingSeriesParams, TrendingSeriesResponse},
        service,
    },
    persistence::models::Session,
};
use axum::{
    Extension, Json,
    extract::{Path, Query},
    http::StatusCode,
};

#[axum::debug_handler]
pub async fn get_trending_movies(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Path(time_window): Path<String>,
    Query(params): Query<TrendingMoviesParams>,
) -> Result<(StatusCode, Json<TrendingMoviesResponse>), AppError> {
    match service::get_trending_movies(app_state.tmdb_client, time_window, params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(app_error) => Err(app_error),
    }
}

#[axum::debug_handler]
pub async fn get_trending_series(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Path(time_window): Path<String>,
    Query(params): Query<TrendingSeriesParams>,
) -> Result<(StatusCode, Json<TrendingSeriesResponse>), AppError> {
    match service::get_trending_series(app_state.tmdb_client, time_window, params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(app_error) => Err(app_error),
    }
}
