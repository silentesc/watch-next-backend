use axum::{
    Extension, Json,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::{
    app::errors::AppError,
    app::state::AppState,
    features::movies::{dto::*, service},
    persistence::models::Session,
};

#[axum::debug_handler]
pub async fn get_movie_details(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Path(movie_id): Path<i32>,
    Query(params): Query<MovieDetailsParams>,
) -> Result<(StatusCode, Json<MovieDetails>), AppError> {
    Ok((
        StatusCode::OK,
        Json(service::get_details(&app_state.tmdb_client, movie_id, &params).await?),
    ))
}

#[axum::debug_handler]
pub async fn get_movie_credits(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Path(movie_id): Path<i32>,
    Query(params): Query<MovieCreditsParams>,
) -> Result<(StatusCode, Json<MovieCreditsResponse>), AppError> {
    Ok((
        StatusCode::OK,
        Json(service::get_credits(&app_state.tmdb_client, movie_id, &params).await?),
    ))
}

#[axum::debug_handler]
pub async fn get_movie_recommendations(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Path(movie_id): Path<i32>,
    Query(params): Query<MovieRecommendationsParams>,
) -> Result<(StatusCode, Json<MovieRecommendationsResponse>), AppError> {
    Ok((
        StatusCode::OK,
        Json(service::get_recommendations(&app_state.tmdb_client, movie_id, &params).await?),
    ))
}

#[axum::debug_handler]
pub async fn get_movie_release_dates(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Path(movie_id): Path<i32>,
) -> Result<(StatusCode, Json<MovieReleaseDatesResponse>), AppError> {
    Ok((
        StatusCode::OK,
        Json(service::get_release_dates(&app_state.tmdb_client, movie_id).await?),
    ))
}

#[axum::debug_handler]
pub async fn get_similar_movies(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Path(movie_id): Path<i32>,
    Query(params): Query<SimilarMoviesParams>,
) -> Result<(StatusCode, Json<SimilarMoviesResponse>), AppError> {
    Ok((
        StatusCode::OK,
        Json(service::get_similar(&app_state.tmdb_client, movie_id, &params).await?),
    ))
}

#[axum::debug_handler]
pub async fn get_movie_videos(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Path(movie_id): Path<i32>,
    Query(params): Query<MovieVideosParams>,
) -> Result<(StatusCode, Json<MovieVideosResponse>), AppError> {
    Ok((
        StatusCode::OK,
        Json(service::get_videos(&app_state.tmdb_client, movie_id, &params).await?),
    ))
}
