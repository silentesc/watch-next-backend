use axum::{Extension, Json, extract::Query, http::StatusCode};

use crate::{
    app::{errors::AppError, state::AppState},
    features::genres::{
        dto::{GenreMovieParams, GenreMovieResponse, GenreTvParams, GenreTvResponse},
        service,
    },
    persistence::models::Session,
};

#[axum::debug_handler]
pub async fn get_movie_genres(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Query(params): Query<GenreMovieParams>,
) -> Result<(StatusCode, Json<GenreMovieResponse>), AppError> {
    match service::get_movie_genres(app_state.tmdb_client, params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(app_error) => Err(app_error),
    }
}

#[axum::debug_handler]
pub async fn get_tv_genres(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Query(params): Query<GenreTvParams>,
) -> Result<(StatusCode, Json<GenreTvResponse>), AppError> {
    match service::get_tv_genres(app_state.tmdb_client, params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(app_error) => Err(app_error),
    }
}
