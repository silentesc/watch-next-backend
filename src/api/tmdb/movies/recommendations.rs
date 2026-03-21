use crate::{
    api::{
        errors::AppError,
        handlers::movies::params::MovieRecommendationsParams,
        tmdb::{movies::models::MovieRecommendationsResponse, utils},
    },
    error,
    logger::enums::category::Category,
};
use reqwest::Client;
use std::collections::HashSet;

pub async fn get_movie_recommendations(
    client: Client,
    movie_id: i32,
    params: MovieRecommendationsParams,
) -> Result<MovieRecommendationsResponse, AppError> {
    // Make request
    let endpoint_url = match utils::parse_url(format!("/movie/{}/recommendations", movie_id).as_str()) {
        Ok(url) => url,
        Err(app_error) => return Err(app_error),
    };
    let request_builder = client.get(endpoint_url).query(&params);
    let response = match utils::send_request(request_builder).await {
        Ok(response) => response,
        Err(app_error) => return Err(app_error),
    };

    // Parse response
    let movie_recommendations_response: MovieRecommendationsResponse = match response.json().await {
        Ok(response) => response,
        Err(err) => {
            error!(
                Category::Tmdb,
                "Parsing MovieRecommendationsResponse failed with error: {:#?}", err
            );
            return Err(AppError::generic_500());
        }
    };

    // Clean response
    let mut seen_ids = HashSet::new();
    let mut cleaned_results = Vec::new();

    for movie in movie_recommendations_response.results {
        if seen_ids.insert(movie.id) {
            cleaned_results.push(movie);
        }
    }

    let cleaned_response = MovieRecommendationsResponse {
        total_results: movie_recommendations_response.total_results,
        total_pages: movie_recommendations_response.total_pages,
        page: movie_recommendations_response.page,
        results: cleaned_results,
    };

    Ok(cleaned_response)
}
