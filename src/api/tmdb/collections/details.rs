use crate::{
    api::{
        errors::AppError,
        handlers::collections::params::CollectionDetailsParams,
        tmdb::{models::CollectionDetails, utils},
    },
    error,
    logger::enums::category::Category,
};
use reqwest::Client;

pub async fn get_collection_details(
    client: Client,
    collection_id: i32,
    params: CollectionDetailsParams,
) -> Result<CollectionDetails, AppError> {
    // Make request
    let endpoint_url = match utils::parse_url(format!("/collection/{}", collection_id).as_str()) {
        Ok(url) => url,
        Err(app_error) => return Err(app_error),
    };
    let request_builder = client.get(endpoint_url).query(&params);
    let response = match utils::send_request(request_builder).await {
        Ok(response) => response,
        Err(app_error) => return Err(app_error),
    };

    // Parse response
    match response.json().await {
        Ok(response) => Ok(response),
        Err(err) => {
            error!(
                Category::Tmdb,
                "Parsing CollectionDetails failed with error: {:#?}", err
            );
            Err(AppError::generic_500())
        }
    }
}
