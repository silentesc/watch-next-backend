use reqwest::{RequestBuilder, Response, StatusCode, Url};

use crate::{
    api::{errors::AppError, tmdb::constants},
    error,
    logger::enums::category::Category,
};

/**
 * Parse any endpoint to a url that can be requested from
 * Example endpoint: /discover/movie
 */
pub fn parse_url(endpoint: &str) -> Result<Url, AppError> {
    let url_string = format!("{}{}", constants::BASE_URL, endpoint);
    match Url::parse(&url_string) {
        Ok(url) => Ok(url),
        Err(err) => {
            error!(
                Category::Tmdb,
                "Parsing {} to url failed with error: {:#?}", url_string, err
            );
            Err(AppError::generic_500())
        }
    }
}

/**
 * Send request to tmdb and handle responses and logging
 */
pub async fn send_request(request_builder: RequestBuilder) -> Result<Response, AppError> {
    match request_builder.send().await {
        Ok(response) => {
            // If success
            if response.status().is_success() {
                return Ok(response);
            }

            let response_status = response.status();
            let response_text = response
                .text()
                .await
                .unwrap_or_else(|_| String::from("Failed to get response text"));

            // Handle 400
            if response_status == StatusCode::BAD_REQUEST {
                return Err(AppError::new(StatusCode::BAD_REQUEST, response_text));
            }

            // Handle 429
            if response_status == StatusCode::TOO_MANY_REQUESTS {
                return Err(AppError::new(StatusCode::TOO_MANY_REQUESTS, response_text));
            }

            error!(
                Category::Tmdb,
                "Request failed with status code {}: {}", response_status, response_text
            );
            Err(AppError::generic_500())
        }
        Err(err) => {
            error!(Category::Tmdb, "Sending request failed with error: {:#?}", err);
            Err(AppError::generic_500())
        }
    }
}
