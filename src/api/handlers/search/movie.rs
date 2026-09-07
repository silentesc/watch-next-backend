use axum::{Extension, Json, extract::Query, http::StatusCode};

use crate::{
    api::{
        db::models::Session,
        errors::AppError,
        models::search::{requests::SearchMovieParams, responses::SearchMovieResponse},
        services,
    },
    state::AppState,
};

#[axum::debug_handler]
pub async fn search_movie(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Query(params): Query<SearchMovieParams>,
) -> Result<(StatusCode, Json<SearchMovieResponse>), AppError> {
    match services::search::movie::search_movie(app_state.tmdb_client, params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(app_error) => Err(app_error),
    }
}
