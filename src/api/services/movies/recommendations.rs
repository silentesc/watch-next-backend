use reqwest::Client;

use crate::api::{
    errors::AppError,
    handlers::movies::params::MovieRecommendationsParams,
    tmdb::{self, movies::models::MovieRecommendationsResponse},
};

pub async fn get_movie_recommendations(
    client: Client,
    movie_id: i32,
    params: MovieRecommendationsParams,
) -> Result<MovieRecommendationsResponse, AppError> {
    match tmdb::movies::recommendations::get_movie_recommendations(client, movie_id, params).await {
        Ok(response) => Ok(response),
        Err(app_error) => Err(app_error),
    }
}
