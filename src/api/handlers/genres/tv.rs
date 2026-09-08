use axum::{Extension, Json, extract::Query, http::StatusCode};

use crate::{
    api::{
        db::models::Session,
        errors::AppError,
        models::genres::{requests::GenreTvParams, responses::GenreTvResponse},
        services,
    },
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_tv_genres(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Query(params): Query<GenreTvParams>,
) -> Result<(StatusCode, Json<GenreTvResponse>), AppError> {
    match services::genres::tv::get_tv_genres(app_state.tmdb_client, params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(app_error) => Err(app_error),
    }
}
