use axum::{Extension, Json, http::StatusCode};

use crate::{
    api::{db::models::Session, errors::AppError, services, tmdb::models::Language},
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_languages(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
) -> Result<(StatusCode, Json<Vec<Language>>), AppError> {
    match services::configuration::languages::get_languages(app_state.client).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(app_error) => Err(app_error),
    }
}
