use reqwest::Client;

use crate::api::{
    errors::AppError,
    handlers::movies::params::MovieDetailsParams,
    tmdb::{self, models::MovieDetails},
};

pub async fn get_movie_details(
    client: Client,
    movie_id: i32,
    params: MovieDetailsParams,
) -> Result<MovieDetails, AppError> {
    match tmdb::movies::details::get_movie_details(client, movie_id, params).await {
        Ok(response) => Ok(response),
        Err(app_error) => Err(app_error),
    }
}
