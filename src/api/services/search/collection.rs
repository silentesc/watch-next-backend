use reqwest::Client;

use crate::api::{
    errors::AppError,
    handlers::search::params::SearchCollectionParams,
    tmdb::{self, search::models::SearchCollectionResponse},
};

pub async fn search_collection(
    client: Client,
    params: SearchCollectionParams,
) -> Result<SearchCollectionResponse, AppError> {
    match tmdb::search::collection::search_collection(client, params).await {
        Ok(response) => Ok(response),
        Err(app_error) => Err(app_error),
    }
}
