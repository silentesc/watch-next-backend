use crate::{
    app::errors::AppError,
    features::genres::dto::{GenreMovieParams, GenreMovieResponse, GenreTvParams, GenreTvResponse},
    integrations::tmdb::client::TmdbClient,
};

pub async fn get_movie_genres(client: TmdbClient, params: GenreMovieParams) -> Result<GenreMovieResponse, AppError> {
    client.get("/genre/movie/list", &params).await.map_err(Into::into)
}

pub async fn get_tv_genres(client: TmdbClient, params: GenreTvParams) -> Result<GenreTvResponse, AppError> {
    client.get("/genre/tv/list", &params).await.map_err(Into::into)
}
