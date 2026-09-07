use crate::api::{
    errors::AppError,
    handlers::movies::params::SimilarMoviesParams,
    tmdb::{client::TmdbClient, movies::models::SimilarMoviesResponse},
};

pub async fn get_similar_movies(
    client: TmdbClient,
    movie_id: i32,
    params: SimilarMoviesParams,
) -> Result<SimilarMoviesResponse, AppError> {
    let mut response: SimilarMoviesResponse = client
        .get(format!("/movie/{movie_id}/similar").as_str(), &params)
        .await?;

    let mut seen_ids = std::collections::HashSet::new();
    response.results.retain(|movie| seen_ids.insert(movie.id));
    Ok(response)
}
