use crate::{
    api::{
        errors::AppError,
        tmdb::{movies::models::MovieReleaseDatesResponse, utils},
    },
    error,
    logger::enums::category::Category,
};
use reqwest::Client;

pub async fn get_movie_release_dates(client: Client, movie_id: i32) -> Result<MovieReleaseDatesResponse, AppError> {
    // Make request
    let endpoint_url = match utils::parse_url(format!("/movie/{}/release_dates", movie_id).as_str()) {
        Ok(url) => url,
        Err(app_error) => return Err(app_error),
    };
    let request_builder = client.get(endpoint_url);
    let response = match utils::send_request(request_builder).await {
        Ok(response) => response,
        Err(app_error) => return Err(app_error),
    };

    // Parse response
    match response.json().await {
        Ok(response) => Ok(response),
        Err(err) => {
            error!(Category::Tmdb, "Parsing MovieReleaseDatesResponse failed with error: {:#?}", err);
            Err(AppError::generic_500())
        }
    }
}
