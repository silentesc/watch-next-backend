use crate::{
    app::{errors::AppError, state::AppState},
    features::tv_series::{dto::*, service},
    persistence::models::Session,
};
use axum::{
    Extension, Json,
    extract::{Path, Query},
    http::StatusCode,
};

#[axum::debug_handler]
pub async fn get_series_details(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Path(series_id): Path<i32>,
    Query(params): Query<TvSeriesDetailsParams>,
) -> Result<(StatusCode, Json<TvSeriesDetails>), AppError> {
    Ok((
        StatusCode::OK,
        Json(service::get_series_details(app_state.tmdb_client, series_id, params).await?),
    ))
}

#[axum::debug_handler]
pub async fn get_series_recommendations(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Path(series_id): Path<i32>,
    Query(params): Query<TvSeriesRecommendationsParams>,
) -> Result<(StatusCode, Json<TvSeriesRecommendationsResponse>), AppError> {
    Ok((
        StatusCode::OK,
        Json(service::get_series_recommendations(app_state.tmdb_client, series_id, params).await?),
    ))
}

#[axum::debug_handler]
pub async fn get_similar_series(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Path(series_id): Path<i32>,
    Query(params): Query<SimilarTvSeriesParams>,
) -> Result<(StatusCode, Json<SimilarTvSeriesResponse>), AppError> {
    Ok((
        StatusCode::OK,
        Json(service::get_similar_series(app_state.tmdb_client, series_id, params).await?),
    ))
}

#[axum::debug_handler]
pub async fn get_series_videos(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Path(series_id): Path<i32>,
    Query(params): Query<TvSeriesVideosParams>,
) -> Result<(StatusCode, Json<TvSeriesVideosResponse>), AppError> {
    Ok((
        StatusCode::OK,
        Json(service::get_series_videos(app_state.tmdb_client, series_id, params).await?),
    ))
}
