use std::time::Duration;

use axum::http::{HeaderMap, HeaderValue};
use reqwest::{
    Client as HttpClient,
    header::{ACCEPT, AUTHORIZATION},
};
use serde::{Serialize, de::DeserializeOwned};

use crate::integrations::tmdb::errors::TmdbError;

const BASE_URL: &str = "https://api.themoviedb.org/3";

#[derive(Clone)]
pub struct TmdbClient {
    http_client: HttpClient,
    base_url: String,
}

impl TmdbClient {
    pub fn new(access_token: String) -> Result<Self, TmdbError> {
        let mut headers = HeaderMap::new();
        headers.append(
            AUTHORIZATION,
            HeaderValue::try_from(format!("Bearer {access_token}"))
                .map_err(|err| TmdbError::InvalidConfiguration { error: err.to_string() })?,
        );
        headers.append(ACCEPT, HeaderValue::from_static("application/json"));

        let http_client = HttpClient::builder()
            .timeout(Duration::from_secs(10))
            .connect_timeout(Duration::from_secs(10))
            .default_headers(headers)
            .build()
            .map_err(|err| TmdbError::Http { error: err.to_string() })?;

        Ok(Self {
            http_client,
            base_url: BASE_URL.to_string(),
        })
    }

    pub async fn get<T, Q>(&self, endpoint: &str, query: &Q) -> Result<T, TmdbError>
    where
        T: DeserializeOwned,
        Q: Serialize + ?Sized,
    {
        let endpoint_url = format!("{}{}", self.base_url, endpoint);

        let response = self
            .http_client
            .get(endpoint_url)
            .query(query)
            .send()
            .await
            .map_err(|err| TmdbError::Http { error: err.to_string() })?;

        let status = response.status();

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(TmdbError::Api { status, body });
        }

        response
            .json()
            .await
            .map_err(|err| TmdbError::Json { error: err.to_string() })
    }
}
