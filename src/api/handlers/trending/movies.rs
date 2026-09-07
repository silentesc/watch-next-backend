use axum::{
    Extension, Json,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::{
    api::{
        db::models::Session,
        errors::AppError,
        models::trending::{requests::TrendingMoviesParams, responses::TrendingMoviesResponse},
        services,
    },
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_trending_movies(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Path(time_window): Path<String>,
    Query(params): Query<TrendingMoviesParams>,
) -> Result<(StatusCode, Json<TrendingMoviesResponse>), AppError> {
    match services::trending::movies::get_trending_movies(app_state.tmdb_client, time_window, params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(app_error) => Err(app_error),
    }
}
