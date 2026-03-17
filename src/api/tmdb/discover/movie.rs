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
use std::collections::HashSet;

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
    let discover_movie_response: DiscoverMovieResponse = match response.json().await {
        Ok(response) => response,
        Err(err) => {
            error!(
                Category::Tmdb,
                "Parsing DiscoverMovieResponse failed with error: {:#?}", err
            );
            return Err(AppError::generic_500());
        }
    };

    // Clean response
    let mut seen_ids = HashSet::new();
    let mut cleaned_results = Vec::new();

    for movie in discover_movie_response.results {
        if seen_ids.insert(movie.id) {
            cleaned_results.push(movie);
        }
    }

    let cleaned_response = DiscoverMovieResponse {
        total_results: discover_movie_response.total_results,
        total_pages: discover_movie_response.total_pages,
        page: discover_movie_response.page,
        results: cleaned_results,
    };

    Ok(cleaned_response)
}
