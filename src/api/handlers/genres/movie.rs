use axum::{Extension, extract::Query, http::StatusCode};

use crate::{
    api::{
        db::models::Session, errors::AppError, handlers::genres::params::GenreMovieParams, services,
        tmdb::genres::models::GenreMovieResponse,
    },
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_movie_genres(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Query(params): Query<GenreMovieParams>,
) -> Result<(StatusCode, GenreMovieResponse), AppError> {
    match services::genres::movie::get_movie_genres(app_state.client, params).await {
        Ok(response) => Ok((StatusCode::OK, response)),
        Err(app_error) => Err(app_error),
    }
}
