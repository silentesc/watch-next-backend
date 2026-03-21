use axum::{Extension, extract::Path, http::StatusCode};

use crate::{
    api::{db::models::Session, errors::AppError, services, tmdb::movies::models::MovieReleaseDatesResponse},
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_movie_release_dates(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Path(movie_id): Path<i32>,
) -> Result<(StatusCode, MovieReleaseDatesResponse), AppError> {
    match services::movies::release_dates::get_movie_release_dates(app_state.client, movie_id).await {
        Ok(response) => Ok((StatusCode::OK, response)),
        Err(app_error) => Err(app_error),
    }
}
