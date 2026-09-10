use axum::{
    Extension, Json,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::{
    api::{
        db::models::Session,
        errors::AppError,
        models::trending::{requests::TrendingSeriesParams, responses::TrendingSeriesResponse},
        services,
    },
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_trending_series(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Path(time_window): Path<String>,
    Query(params): Query<TrendingSeriesParams>,
) -> Result<(StatusCode, Json<TrendingSeriesResponse>), AppError> {
    match services::trending::tv::get_trending_series(app_state.tmdb_client, time_window, params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(app_error) => Err(app_error),
    }
}
