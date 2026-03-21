use reqwest::Client;

use crate::api::{
    errors::AppError,
    handlers::trending::params::TrendingMoviesParams,
    tmdb::{self, trending::models::TrendingMoviesResponse},
};

pub async fn get_trending_movies(
    client: Client,
    time_window: String,
    params: TrendingMoviesParams,
) -> Result<TrendingMoviesResponse, AppError> {
    match tmdb::trending::movies::get_trending_movies(client, time_window, params).await {
        Ok(response) => Ok(response),
        Err(app_error) => Err(app_error),
    }
}
