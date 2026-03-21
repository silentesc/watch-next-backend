use std::collections::HashSet;

use crate::{
    api::{
        errors::AppError,
        handlers::search::params::SearchCollectionParams,
        tmdb::{search::models::SearchCollectionResponse, utils},
    },
    error,
    logger::enums::category::Category,
};
use reqwest::Client;

pub async fn search_collection(
    client: Client,
    params: SearchCollectionParams,
) -> Result<SearchCollectionResponse, AppError> {
    // Make request
    let endpoint_url = match utils::parse_url("/search/collection") {
        Ok(url) => url,
        Err(app_error) => return Err(app_error),
    };
    let request_builder = client.get(endpoint_url).query(&params);
    let response = match utils::send_request(request_builder).await {
        Ok(response) => response,
        Err(app_error) => return Err(app_error),
    };

    // Parse response
    let search_collection_response: SearchCollectionResponse = match response.json().await {
        Ok(response) => response,
        Err(err) => {
            error!(
                Category::Tmdb,
                "Parsing SearchCollectionResponse failed with error: {:#?}", err
            );
            return Err(AppError::generic_500());
        }
    };

    // Clean response
    let mut seen_ids = HashSet::new();
    let mut cleaned_results = Vec::new();

    for collection_overview in search_collection_response.results {
        if seen_ids.insert(collection_overview.id) {
            cleaned_results.push(collection_overview);
        }
    }

    let cleaned_response = SearchCollectionResponse {
        total_results: search_collection_response.total_results,
        total_pages: search_collection_response.total_pages,
        page: search_collection_response.page,
        results: cleaned_results,
    };

    Ok(cleaned_response)
}
