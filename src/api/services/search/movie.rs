use reqwest::Client;

use crate::api::{
    errors::AppError,
    handlers::search::params::SearchMovieParams,
    tmdb::{self, search::models::SearchMovieResponse},
};

pub async fn search_movie(client: Client, params: SearchMovieParams) -> Result<SearchMovieResponse, AppError> {
    match tmdb::search::movie::search_movie(client, params).await {
        Ok(response) => Ok(response),
        Err(app_error) => Err(app_error),
    }
}
