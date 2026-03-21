use crate::{
    api::{
        errors::AppError,
        handlers::movies::params::SimilarMoviesParams,
        tmdb::{movies::models::SimilarMoviesResponse, utils},
    },
    error,
    logger::enums::category::Category,
};
use reqwest::Client;
use std::collections::HashSet;

pub async fn get_similar_movies(
    client: Client,
    movie_id: i32,
    params: SimilarMoviesParams,
) -> Result<SimilarMoviesResponse, AppError> {
    // Make request
    let endpoint_url = match utils::parse_url(format!("/movie/{}/similar", movie_id).as_str()) {
        Ok(url) => url,
        Err(app_error) => return Err(app_error),
    };
    let request_builder = client.get(endpoint_url).query(&params);
    let response = match utils::send_request(request_builder).await {
        Ok(response) => response,
        Err(app_error) => return Err(app_error),
    };

    // Parse response
    let similar_movies_response: SimilarMoviesResponse = match response.json().await {
        Ok(response) => response,
        Err(err) => {
            error!(
                Category::Tmdb,
                "Parsing SimilarMoviesResponse failed with error: {:#?}", err
            );
            return Err(AppError::generic_500());
        }
    };

    // Clean response
    let mut seen_ids = HashSet::new();
    let mut cleaned_results = Vec::new();

    for movie in similar_movies_response.results {
        if seen_ids.insert(movie.id) {
            cleaned_results.push(movie);
        }
    }

    let cleaned_response = SimilarMoviesResponse {
        total_results: similar_movies_response.total_results,
        total_pages: similar_movies_response.total_pages,
        page: similar_movies_response.page,
        results: cleaned_results,
    };

    Ok(cleaned_response)
}
