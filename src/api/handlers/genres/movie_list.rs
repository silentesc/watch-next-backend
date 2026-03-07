use axum::{Extension, extract::Query, http::StatusCode};

use crate::{
    api::{
        db::models::Session, errors::AppError, handlers::genres::params::GenreMovieListParams, services,
        tmdb::genres::models::GenreMovieListResponse,
    },
    state::AppState,
};

#[axum::debug_handler]
pub async fn movie_list(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Query(params): Query<GenreMovieListParams>,
) -> Result<(StatusCode, GenreMovieListResponse), AppError> {
    match services::genres::movie_list::movie_list(app_state.client, params).await {
        Ok(response) => Ok((StatusCode::OK, response)),
        Err(app_error) => Err(app_error),
    }
}
