use axum::{
    Extension, Json,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::{
    api::{
        db::models::Session,
        errors::AppError,
        models::movies::{requests::MovieVideosParams, responses::MovieVideosResponse},
        services,
    },
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_movie_videos(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Path(movie_id): Path<i32>,
    Query(params): Query<MovieVideosParams>,
) -> Result<(StatusCode, Json<MovieVideosResponse>), AppError> {
    match services::movies::videos::get_movie_videos(app_state.tmdb_client, movie_id, params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(app_error) => Err(app_error),
    }
}
