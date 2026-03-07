use reqwest::Client;

use crate::api::{
    errors::AppError,
    handlers::genres::params::GenreMovieListParams,
    tmdb::{self, genres::models::GenreMovieListResponse},
};

pub async fn movie_list(client: Client, params: GenreMovieListParams) -> Result<GenreMovieListResponse, AppError> {
    match tmdb::genres::movie_list::movie_list(client, params).await {
        Ok(response) => Ok(response),
        Err(app_error) => Err(app_error),
    }
}
