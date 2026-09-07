use axum::{Extension, Json, extract::Query, http::StatusCode};

use crate::{
    api::{
        db::models::Session,
        errors::AppError,
        models::search::{requests::SearchCollectionParams, responses::SearchCollectionResponse},
        services,
    },
    state::AppState,
};

#[axum::debug_handler]
pub async fn search_collection(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Query(params): Query<SearchCollectionParams>,
) -> Result<(StatusCode, Json<SearchCollectionResponse>), AppError> {
    match services::search::collection::search_collection(app_state.tmdb_client, params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(app_error) => Err(app_error),
    }
}
