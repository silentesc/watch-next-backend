use axum::{
    Extension, Json,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::{
    api::{
        db::models::Session,
        errors::AppError,
        models::tv::{requests::TvDetailsParams, responses::TvDetails},
        services,
    },
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_series_details(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Path(series_id): Path<i32>,
    Query(params): Query<TvDetailsParams>,
) -> Result<(StatusCode, Json<TvDetails>), AppError> {
    match services::tv::details::get_series_details(app_state.tmdb_client, series_id, params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(app_error) => Err(app_error),
    }
}
