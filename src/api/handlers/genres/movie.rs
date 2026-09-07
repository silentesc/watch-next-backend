use axum::{Extension, Json, extract::Query, http::StatusCode};

use crate::{
    api::{
        db::models::Session,
        errors::AppError,
        models::genres::{requests::GenreMovieParams, responses::GenreMovieResponse},
        services,
    },
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_movie_genres(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Query(params): Query<GenreMovieParams>,
) -> Result<(StatusCode, Json<GenreMovieResponse>), AppError> {
    match services::genres::movie::get_movie_genres(app_state.tmdb_client, params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(app_error) => Err(app_error),
    }
}
