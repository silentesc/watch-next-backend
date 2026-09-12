use axum::{Extension, Json, extract::Query, http::StatusCode};

use crate::{
    app::{errors::AppError, state::AppState},
    features::discover::service,
    integrations::tmdb::resources::{
        movies::dto::{DiscoverMovieParams, DiscoverMovieResponse},
        tv_series::dto::{DiscoverTvSeriesParams, DiscoverTvSeriesResponse},
    },
    persistence::models::Session,
};

#[axum::debug_handler]
pub async fn discover_movies(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Query(params): Query<DiscoverMovieParams>,
) -> Result<(StatusCode, Json<DiscoverMovieResponse>), AppError> {
    match service::discover_movies(app_state.tmdb, params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(app_error) => Err(app_error),
    }
}

#[axum::debug_handler]
pub async fn discover_tv_series(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Query(params): Query<DiscoverTvSeriesParams>,
) -> Result<(StatusCode, Json<DiscoverTvSeriesResponse>), AppError> {
    match service::discover_tv_series(app_state.tmdb, params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(app_error) => Err(app_error),
    }
}
