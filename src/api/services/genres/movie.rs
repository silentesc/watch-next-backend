use crate::api::{
    errors::AppError,
    models::genres::{requests::GenreMovieParams, responses::GenreMovieResponse},
    tmdb::client::TmdbClient,
};

pub async fn get_movie_genres(client: TmdbClient, params: GenreMovieParams) -> Result<GenreMovieResponse, AppError> {
    client.get("/genre/movie/list", &params).await
}
