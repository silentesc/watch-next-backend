use crate::{
    api::{
        errors::AppError,
        handlers::search::params::SearchMovieParams,
        tmdb::{search::models::SearchMovieResponse, utils},
    },
    error,
    logger::enums::category::Category,
};
use reqwest::Client;

pub async fn search_movie(client: Client, params: SearchMovieParams) -> Result<SearchMovieResponse, AppError> {
    // Make request
    let endpoint_url = match utils::parse_url("/search/movie") {
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
                "Parsing SearchMovieResponse failed with error: {:#?}", err
            );
            Err(AppError::generic_500())
        }
    }
}
