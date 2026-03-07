use reqwest::Client;

use crate::api::{
    errors::AppError,
    handlers::genres::params::GenreMovieParams,
    tmdb::{self, genres::models::GenreMovieResponse},
};

pub async fn get_movie_genres(client: Client, params: GenreMovieParams) -> Result<GenreMovieResponse, AppError> {
    match tmdb::genres::movie::get_movie_genres(client, params).await {
        Ok(response) => Ok(response),
        Err(app_error) => Err(app_error),
    }
}
