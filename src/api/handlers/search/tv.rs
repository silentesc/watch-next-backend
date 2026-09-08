use axum::{Extension, Json, extract::Query, http::StatusCode};

use crate::{
    api::{
        db::models::Session,
        errors::AppError,
        models::search::{requests::SearchTvParams, responses::SearchTvResponse},
        services,
    },
    state::AppState,
};

#[axum::debug_handler]
pub async fn search_show(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Query(params): Query<SearchTvParams>,
) -> Result<(StatusCode, Json<SearchTvResponse>), AppError> {
    match services::search::tv::search_show(app_state.tmdb_client, params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(app_error) => Err(app_error),
    }
}
