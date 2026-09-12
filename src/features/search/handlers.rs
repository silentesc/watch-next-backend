use crate::{
    app::{errors::AppError, state::AppState},
    features::search::{
        dto::{
            SearchCollectionParams, SearchCollectionResponse, SearchMovieParams, SearchMovieResponse, SearchTvParams,
            SearchTvResponse,
        },
        service,
    },
    persistence::models::Session,
};
use axum::{Extension, Json, extract::Query, http::StatusCode};

#[axum::debug_handler]
pub async fn search_collection(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Query(params): Query<SearchCollectionParams>,
) -> Result<(StatusCode, Json<SearchCollectionResponse>), AppError> {
    match service::search_collection(app_state.tmdb_client, params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(app_error) => Err(app_error),
    }
}

#[axum::debug_handler]
pub async fn search_movie(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Query(params): Query<SearchMovieParams>,
) -> Result<(StatusCode, Json<SearchMovieResponse>), AppError> {
    match service::search_movie(app_state.tmdb_client, params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(app_error) => Err(app_error),
    }
}

#[axum::debug_handler]
pub async fn search_series(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Query(params): Query<SearchTvParams>,
) -> Result<(StatusCode, Json<SearchTvResponse>), AppError> {
    match service::search_series(app_state.tmdb_client, params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(app_error) => Err(app_error),
    }
}
