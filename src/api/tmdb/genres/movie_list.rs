use crate::{
    api::{
        errors::AppError,
        handlers::genres::params::GenreMovieListParams,
        tmdb::{genres::models::GenreMovieListResponse, utils},
    },
    error,
    logger::enums::category::Category,
};
use reqwest::Client;

pub async fn movie_list(client: Client, params: GenreMovieListParams) -> Result<GenreMovieListResponse, AppError> {
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
                "Parsing GenreMovieListResponse failed with error: {:#?}", err
            );
            Err(AppError::generic_500())
        }
    }
}
