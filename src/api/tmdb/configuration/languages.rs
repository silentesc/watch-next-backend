use crate::{
    api::{
        errors::AppError,
        tmdb::{models::Language, utils},
    },
    error,
    logger::enums::category::Category,
};
use reqwest::Client;

pub async fn get_languages(client: Client) -> Result<Vec<Language>, AppError> {
    // Make request
    let endpoint_url = match utils::parse_url("/configuration/languages") {
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
            error!(
                Category::Tmdb,
                "Parsing ConfigurationLanguagesResponse failed with error: {:#?}", err
            );
            Err(AppError::generic_500())
        }
    }
}
