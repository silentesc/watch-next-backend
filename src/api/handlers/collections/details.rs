use axum::{
    Extension, Json,
    extract::{Path, Query},
    http::StatusCode,
};

use crate::{
    api::{
        db::models::Session,
        errors::AppError,
        models::collections::{requests::CollectionDetailsParams, responses::CollectionDetails},
        services,
    },
    state::AppState,
};

#[axum::debug_handler]
pub async fn get_collection_details(
    Extension(app_state): Extension<AppState>,
    Extension(_): Extension<Session>,
    Path(collection_id): Path<i32>,
    Query(params): Query<CollectionDetailsParams>,
) -> Result<(StatusCode, Json<CollectionDetails>), AppError> {
    match services::collections::details::get_collection_details(app_state.tmdb_client, collection_id, params).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(app_error) => Err(app_error),
    }
}
