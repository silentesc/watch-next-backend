use axum::{Extension, extract::Query, http::StatusCode};

use crate::{
    api::{
        db::models::Session, errors::AppError, handlers::discover::params::DiscoverMovieParams, services,
        tmdb::discover::models::DiscoverMovieResponse,
    },
    state::AppState,
};

#[axum::debug_handler]
pub async fn discover(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Query(params): Query<DiscoverMovieParams>,
) -> Result<(StatusCode, DiscoverMovieResponse), AppError> {
    match services::discover::movie::discover(app_state.client, params).await {
        Ok(response) => Ok((StatusCode::OK, response)),
        Err(app_error) => Err(app_error),
    }
}
