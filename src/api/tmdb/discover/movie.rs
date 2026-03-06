use crate::{
    api::{
        errors::AppError,
        handlers::discover::params::DiscoverMovieParams,
        tmdb::{discover::models::DiscoverMovieResponse, utils},
    },
    error,
    logger::enums::category::Category,
};
use reqwest::Client;

pub async fn discover(client: Client, params: DiscoverMovieParams) -> Result<DiscoverMovieResponse, AppError> {
    // Make request
    let endpoint_url = match utils::parse_url("/discover/movie") {
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
                "Parsing DiscoverMovieResponse failed with error: {:#?}", err
            );
            Err(AppError::generic_500())
        }
    }
}
