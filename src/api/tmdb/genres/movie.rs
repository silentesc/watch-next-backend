use crate::{
    api::{
        errors::AppError,
        handlers::genres::params::GenreMovieParams,
        tmdb::{genres::models::GenreMovieResponse, utils},
    },
    error,
    logger::enums::category::Category,
};
use reqwest::Client;

pub async fn get_movie_genres(client: Client, params: GenreMovieParams) -> Result<GenreMovieResponse, AppError> {
    // Make request
    let endpoint_url = match utils::parse_url("/genre/movie/list") {
        Ok(url) => url,
        Err(app_error) => return Err(app_error),
    };
    let request_builder = client.get(endpoint_url).query(&params);
    let response = match utils::send_request(request_builder).await {
        Ok(response) => response,
        Err(app_error) => return Err(app_error),
    };

    // Parse response
    match response.json().await {
        Ok(response) => Ok(response),
        Err(err) => {
            error!(
                Category::Tmdb,
                "Parsing GenreMovieResponse failed with error: {:#?}", err
            );
            Err(AppError::generic_500())
        }
    }
}
