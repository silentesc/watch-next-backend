use axum::{
    Extension, Json,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::{
    app::{errors::AppError, state::AppState},
    features::tv_seasons::dto::{TvSeasonDetails, TvSeasonDetailsParams},
    persistence::models::Session,
};

#[axum::debug_handler]
pub async fn get_season_details(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Path((series_id, season_id)): Path<(i32, i32)>,
    Query(params): Query<TvSeasonDetailsParams>,
) -> Result<(StatusCode, Json<TvSeasonDetails>), AppError> {
    match crate::features::tv_seasons::service::get_season_details(app_state.tmdb_client, series_id, season_id, params)
        .await
    {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(app_error) => Err(app_error),
    }
}
