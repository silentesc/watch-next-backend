use reqwest::Client;

use crate::api::{
    errors::AppError,
    handlers::movies::params::SimilarMoviesParams,
    tmdb::{self, movies::models::SimilarMoviesResponse},
};

pub async fn get_similar_movies(
    client: Client,
    movie_id: i32,
    params: SimilarMoviesParams,
) -> Result<SimilarMoviesResponse, AppError> {
    match tmdb::movies::similar::get_similar_movies(client, movie_id, params).await {
        Ok(response) => Ok(response),
        Err(app_error) => Err(app_error),
    }
}
