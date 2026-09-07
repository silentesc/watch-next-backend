use crate::api::{
    errors::AppError,
    handlers::collections::params::CollectionDetailsParams,
    tmdb::{client::TmdbClient, models::CollectionDetails},
};

pub async fn get_collection_details(
    client: TmdbClient,
    collection_id: i32,
    params: CollectionDetailsParams,
) -> Result<CollectionDetails, AppError> {
    client
        .get(format!("/collection/{collection_id}").as_str(), &params)
        .await
}
