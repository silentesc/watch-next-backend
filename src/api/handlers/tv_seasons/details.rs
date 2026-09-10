use axum::{
    Extension, Json,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::{
    api::{
        db::models::Session,
        errors::AppError,
        models::tv_seasons::{requests::TvSeasonDetailsParams, responses::TvSeasonDetails},
        services,
    },
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_season_details(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Path((series_id, season_id)): Path<(i32, i32)>,
    Query(params): Query<TvSeasonDetailsParams>,
) -> Result<(StatusCode, Json<TvSeasonDetails>), AppError> {
    match services::tv_seasons::details::get_season_details(app_state.tmdb_client, series_id, season_id, params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(app_error) => Err(app_error),
    }
}
