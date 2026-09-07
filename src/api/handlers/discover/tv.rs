use axum::{Extension, Json, extract::Query, http::StatusCode};

use crate::{
    api::{
        db::models::Session,
        errors::AppError,
        models::discover::{requests::DiscoverTvParams, responses::DiscoverTvResponse},
        services,
    },
    state::AppState,
};

#[axum::debug_handler]
pub async fn discover(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Query(params): Query<DiscoverTvParams>,
) -> Result<(StatusCode, Json<DiscoverTvResponse>), AppError> {
    match services::discover::tv::discover(app_state.tmdb_client, params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(app_error) => Err(app_error),
    }
}
