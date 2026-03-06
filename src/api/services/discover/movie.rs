use reqwest::Client;

use crate::api::{
    errors::AppError,
    handlers::discover::params::DiscoverMovieParams,
    tmdb::{self, discover::models::DiscoverMovieResponse},
};

pub async fn discover(client: Client, params: DiscoverMovieParams) -> Result<DiscoverMovieResponse, AppError> {
    match tmdb::discover::movie::discover(client, params).await {
        Ok(response) => Ok(response),
        Err(app_error) => Err(app_error),
    }
}
