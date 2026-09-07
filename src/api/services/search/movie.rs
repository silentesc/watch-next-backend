use crate::api::{
    errors::AppError,
    handlers::search::params::SearchMovieParams,
    tmdb::{client::TmdbClient, search::models::SearchMovieResponse},
};

pub async fn search_movie(client: TmdbClient, params: SearchMovieParams) -> Result<SearchMovieResponse, AppError> {
    let mut response: SearchMovieResponse = client.get("/search/movie", &params).await?;

    let mut seen_ids = std::collections::HashSet::new();
    response.results.retain(|movie| seen_ids.insert(movie.id));
    Ok(response)
}
