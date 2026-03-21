use crate::{
    api::{
        errors::AppError,
        handlers::trending::params::TrendingMoviesParams,
        tmdb::{trending::models::TrendingMoviesResponse, utils},
    },
    error,
    logger::enums::category::Category,
};
use reqwest::Client;
use std::collections::HashSet;

pub async fn get_trending_movies(
    client: Client,
    time_window: String,
    params: TrendingMoviesParams,
) -> Result<TrendingMoviesResponse, AppError> {
    // Make request
    let endpoint_url = match utils::parse_url(format!("/trending/movie/{}", time_window).as_str()) {
        Ok(url) => url,
        Err(app_error) => return Err(app_error),
    };
    let request_builder = client.get(endpoint_url).query(&params);
    let response = match utils::send_request(request_builder).await {
        Ok(response) => response,
        Err(app_error) => return Err(app_error),
    };

    // Parse response
    let trending_movies_response: TrendingMoviesResponse = match response.json().await {
        Ok(response) => response,
        Err(err) => {
            error!(
                Category::Tmdb,
                "Parsing TrendingMoviesResponse failed with error: {:#?}", err
            );
            return Err(AppError::generic_500());
        }
    };

    // Clean response
    let mut seen_ids = HashSet::new();
    let mut cleaned_results = Vec::new();

    for movie in trending_movies_response.results {
        if seen_ids.insert(movie.id) {
            cleaned_results.push(movie);
        }
    }

    let cleaned_response = TrendingMoviesResponse {
        total_results: trending_movies_response.total_results,
        total_pages: trending_movies_response.total_pages,
        page: trending_movies_response.page,
        results: cleaned_results,
    };

    Ok(cleaned_response)
}
