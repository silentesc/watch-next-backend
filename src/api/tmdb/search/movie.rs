use std::collections::HashSet;

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
    let search_movie_response: SearchMovieResponse = match response.json().await {
        Ok(response) => response,
        Err(err) => {
            error!(
                Category::Tmdb,
                "Parsing SearchMovieResponse failed with error: {:#?}", err
            );
            return Err(AppError::generic_500());
        }
    };

    // Clean response
    let mut seen_ids = HashSet::new();
    let mut cleaned_results = Vec::new();

    for movie in search_movie_response.results {
        if seen_ids.insert(movie.id) {
            cleaned_results.push(movie);
        }
    }

    let cleaned_response = SearchMovieResponse {
        total_results: search_movie_response.total_results,
        total_pages: search_movie_response.total_pages,
        page: search_movie_response.page,
        results: cleaned_results,
    };

    Ok(cleaned_response)
}
