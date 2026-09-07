use axum::{
    Extension, Json,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::{
    api::{
        db::models::Session,
        errors::AppError,
        models::movies::{requests::MovieCreditsParams, responses::MovieCreditsResponse},
        services,
    },
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_movie_credits(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Path(movie_id): Path<i32>,
    Query(params): Query<MovieCreditsParams>,
) -> Result<(StatusCode, Json<MovieCreditsResponse>), AppError> {
    match services::movies::credits::get_movie_credits(app_state.tmdb_client, movie_id, params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(app_error) => Err(app_error),
    }
}
