use crate::{
    app::errors::AppError,
    integrations::tmdb::{
        TmdbApi,
        resources::collections::dto::{CollectionDetailsParams, CollectionDetailsResponse},
    },
};

pub async fn get_collection_details(
    tmdb: TmdbApi,
    collection_id: i32,
    params: CollectionDetailsParams,
) -> Result<CollectionDetailsResponse, AppError> {
    tmdb.collections()
        .details(collection_id, params)
        .await
        .map_err(Into::into)
}
