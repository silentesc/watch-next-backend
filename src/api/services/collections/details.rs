use crate::api::{
    errors::AppError,
    models::collections::{requests::CollectionDetailsParams, responses::CollectionDetails},
    tmdb::client::TmdbClient,
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
