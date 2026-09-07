use axum::{Extension, Json, extract::Path, http::StatusCode};

use crate::{
    api::{db::models::Session, errors::AppError, models::movies::responses::MovieReleaseDatesResponse, services},
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_movie_release_dates(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Path(movie_id): Path<i32>,
) -> Result<(StatusCode, Json<MovieReleaseDatesResponse>), AppError> {
    match services::movies::release_dates::get_movie_release_dates(app_state.tmdb_client, movie_id).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(app_error) => Err(app_error),
    }
}
