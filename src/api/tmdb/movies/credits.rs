use crate::{
    api::{
        errors::AppError,
        handlers::movies::params::MovieCreditsParams,
        tmdb::{movies::models::MovieCreditsResponse, utils},
    },
    error,
    logger::enums::category::Category,
};
use reqwest::Client;

pub async fn get_movie_credits(
    client: Client,
    movie_id: i32,
    params: MovieCreditsParams,
) -> Result<MovieCreditsResponse, AppError> {
    // Make request
    let endpoint_url = match utils::parse_url(format!("/movie/{}/credits", movie_id).as_str()) {
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
                "Parsing MovieCreditsResponse failed with error: {:#?}", err
            );
            Err(AppError::generic_500())
        }
    }
}
