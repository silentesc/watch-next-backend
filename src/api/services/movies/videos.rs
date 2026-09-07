use crate::api::{
    errors::AppError,
    handlers::movies::params::MovieVideosParams,
    tmdb::{client::TmdbClient, movies::models::MovieVideosResponse},
};

pub async fn get_movie_videos(
    client: TmdbClient,
    movie_id: i32,
    params: MovieVideosParams,
) -> Result<MovieVideosResponse, AppError> {
    client.get(format!("/movie/{movie_id}/videos").as_str(), &params).await
}
