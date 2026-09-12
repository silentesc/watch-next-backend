use reqwest::{Client, StatusCode, Url};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

use crate::{app::errors::AppError, error, logger::enums::category::Category};

const BASE_URL: &str = "https://api.themoviedb.org/3";

#[derive(Clone)]
pub struct TmdbClient {
    client: Client,
}

impl TmdbClient {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn get<T, Q>(&self, endpoint: &str, query: &Q) -> Result<T, AppError>
    where
        T: DeserializeOwned,
        Q: Serialize + ?Sized,
    {
        let endpoint_url = self.parse_url(endpoint)?;
        let response = self.client.get(endpoint_url).query(query).send().await.map_err(|err| {
            error!(Category::Tmdb, "Sending request failed with error: {:#?}", err);
            AppError::generic_500()
        })?;

        if !response.status().is_success() {
            return Err(Self::parse_error_response(response).await);
        }

        response.json().await.map_err(|err| {
            error!(Category::Tmdb, "Parsing TMDB response failed with error: {:#?}", err);
            AppError::generic_500()
        })
    }

    fn parse_url(&self, endpoint: &str) -> Result<Url, AppError> {
        let url_string = format!("{}{}", BASE_URL, endpoint);
        Url::parse(&url_string).map_err(|err| {
            error!(
                Category::Tmdb,
                "Parsing {} to url failed with error: {:#?}", url_string, err
            );
            AppError::generic_500()
        })
    }

    async fn parse_error_response(response: reqwest::Response) -> AppError {
        let response_status = response.status();
        let response_json: Value = match response.json().await {
            Ok(response_json) => response_json,
            Err(err) => {
                error!(Category::Tmdb, "Failed to get json from response: {:#?}", err);
                return AppError::generic_500();
            }
        };

        let error_text = match response_json.get("status_message") {
            Some(value) => value.to_string(),
            None => {
                error!(Category::Tmdb, "No status_message in response json");
                String::new()
            }
        };

        match response_status {
            StatusCode::BAD_REQUEST | StatusCode::NOT_FOUND | StatusCode::TOO_MANY_REQUESTS => {
                AppError::new(response_status, error_text)
            }
            _ => {
                error!(
                    Category::Tmdb,
                    "Request failed with status code {}: {}", response_status, error_text
                );
                AppError::generic_500()
            }
        }
    }
}
