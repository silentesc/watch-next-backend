use crate::api::{
    errors::AppError,
    handlers::movies::params::MovieCreditsParams,
    tmdb::{client::TmdbClient, movies::models::MovieCreditsResponse},
};

pub async fn get_movie_credits(
    client: TmdbClient,
    movie_id: i32,
    params: MovieCreditsParams,
) -> Result<MovieCreditsResponse, AppError> {
    client.get(format!("/movie/{movie_id}/credits").as_str(), &params).await
}
