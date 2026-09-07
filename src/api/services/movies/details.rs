use crate::api::{
    errors::AppError,
    handlers::movies::params::MovieDetailsParams,
    tmdb::{client::TmdbClient, models::MovieDetails},
};

pub async fn get_movie_details(
    client: TmdbClient,
    movie_id: i32,
    params: MovieDetailsParams,
) -> Result<MovieDetails, AppError> {
    client.get(format!("/movie/{}", movie_id).as_str(), &params).await
}
