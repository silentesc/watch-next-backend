use crate::api::{
    errors::AppError,
    handlers::genres::params::GenreMovieParams,
    tmdb::{client::TmdbClient, genres::models::GenreMovieResponse},
};

pub async fn get_movie_genres(client: TmdbClient, params: GenreMovieParams) -> Result<GenreMovieResponse, AppError> {
    client.get("/genre/movie/list", &params).await
}
