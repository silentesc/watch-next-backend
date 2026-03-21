use reqwest::Client;

use crate::api::{
    errors::AppError,
    handlers::movies::params::MovieCreditsParams,
    tmdb::{self, movies::models::MovieCreditsResponse},
};

pub async fn get_movie_credits(
    client: Client,
    movie_id: i32,
    params: MovieCreditsParams,
) -> Result<MovieCreditsResponse, AppError> {
    match tmdb::movies::credits::get_movie_credits(client, movie_id, params).await {
        Ok(response) => Ok(response),
        Err(app_error) => Err(app_error),
    }
}
