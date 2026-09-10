use axum::{
    Extension, Json,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::{
    api::{
        db::models::Session,
        errors::AppError,
        models::tv_series::{requests::TvSeriesVideosParams, responses::TvSeriesVideosResponse},
        services,
    },
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_series_videos(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Path(season_id): Path<i32>,
    Query(params): Query<TvSeriesVideosParams>,
) -> Result<(StatusCode, Json<TvSeriesVideosResponse>), AppError> {
    match services::tv_series::videos::get_series_videos(app_state.tmdb_client, season_id, params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(app_error) => Err(app_error),
    }
}
