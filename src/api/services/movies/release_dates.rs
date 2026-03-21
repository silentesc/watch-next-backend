use reqwest::Client;

use crate::api::{
    errors::AppError,
    tmdb::{self, movies::models::MovieReleaseDatesResponse},
};

pub async fn get_movie_release_dates(client: Client, movie_id: i32) -> Result<MovieReleaseDatesResponse, AppError> {
    match tmdb::movies::release_dates::get_movie_release_dates(client, movie_id).await {
        Ok(response) => Ok(response),
        Err(app_error) => Err(app_error),
    }
}
