use reqwest::Client;

use crate::api::{
    errors::AppError,
    handlers::collections::params::CollectionDetailsParams,
    tmdb::{self, models::CollectionDetails},
};

pub async fn get_collection_details(
    client: Client,
    collection_id: i32,
    params: CollectionDetailsParams,
) -> Result<CollectionDetails, AppError> {
    match tmdb::collections::details::get_collection_details(client, collection_id, params).await {
        Ok(response) => Ok(response),
        Err(app_error) => Err(app_error),
    }
}
