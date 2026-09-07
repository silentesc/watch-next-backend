use axum::{
    Extension, Json,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::{
    api::{
        db::models::Session,
        errors::AppError,
        models::movies::{requests::MovieRecommendationsParams, responses::MovieRecommendationsResponse},
        services,
    },
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_movie_recommendations(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Path(movie_id): Path<i32>,
    Query(params): Query<MovieRecommendationsParams>,
) -> Result<(StatusCode, Json<MovieRecommendationsResponse>), AppError> {
    match services::movies::recommendations::get_movie_recommendations(app_state.tmdb_client, movie_id, params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(app_error) => Err(app_error),
    }
}
