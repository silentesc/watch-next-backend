use crate::{
    app::{errors::AppError, state::AppState},
    features::search::service,
    integrations::tmdb::resources::{
        collections::dto::{SearchCollectionsParams, SearchCollectionsResponse},
        movies::dto::{SearchMoviesParams, SearchMoviesResponse},
        tv_series::dto::{SearchTvSeriesParams, SearchTvSeriesResponse},
    },
    persistence::models::Session,
};
use axum::{Extension, Json, extract::Query, http::StatusCode};

#[axum::debug_handler]
pub async fn search_collection(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Query(params): Query<SearchCollectionsParams>,
) -> Result<(StatusCode, Json<SearchCollectionsResponse>), AppError> {
    match service::search_collection(app_state.tmdb, params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(app_error) => Err(app_error),
    }
}

#[axum::debug_handler]
pub async fn search_movie(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Query(params): Query<SearchMoviesParams>,
) -> Result<(StatusCode, Json<SearchMoviesResponse>), AppError> {
    match service::search_movie(app_state.tmdb, params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(app_error) => Err(app_error),
    }
}

#[axum::debug_handler]
pub async fn search_series(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Query(params): Query<SearchTvSeriesParams>,
) -> Result<(StatusCode, Json<SearchTvSeriesResponse>), AppError> {
    match service::search_series(app_state.tmdb, params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(app_error) => Err(app_error),
    }
}
