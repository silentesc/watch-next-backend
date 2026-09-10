use axum::{
    Extension, Json,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::{
    api::{
        db::models::Session,
        errors::AppError,
        models::tv_series::{requests::TvSeriesRecommendationsParams, responses::TvSeriesRecommendationsResponse},
        services,
    },
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_series_recommendations(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Path(series_id): Path<i32>,
    Query(params): Query<TvSeriesRecommendationsParams>,
) -> Result<(StatusCode, Json<TvSeriesRecommendationsResponse>), AppError> {
    match services::tv_series::recommendations::get_series_recommendations(app_state.tmdb_client, series_id, params)
        .await
    {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(app_error) => Err(app_error),
    }
}
