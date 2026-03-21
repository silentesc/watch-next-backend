use reqwest::Client;

use crate::api::{
    errors::AppError,
    handlers::movies::params::MovieVideosParams,
    tmdb::{self, movies::models::MovieVideosResponse},
};

pub async fn get_movie_videos(
    client: Client,
    movie_id: i32,
    params: MovieVideosParams,
) -> Result<MovieVideosResponse, AppError> {
    match tmdb::movies::videos::get_movie_videos(client, movie_id, params).await {
        Ok(response) => Ok(response),
        Err(app_error) => Err(app_error),
    }
}
