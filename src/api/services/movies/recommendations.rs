use crate::api::{
    errors::AppError,
    handlers::movies::params::MovieRecommendationsParams,
    tmdb::{client::TmdbClient, movies::models::MovieRecommendationsResponse},
};

pub async fn get_movie_recommendations(
    client: TmdbClient,
    movie_id: i32,
    params: MovieRecommendationsParams,
) -> Result<MovieRecommendationsResponse, AppError> {
    let mut response: MovieRecommendationsResponse = client
        .get(format!("/movie/{movie_id}/recommendations").as_str(), &params)
        .await?;

    let mut seen_ids = std::collections::HashSet::new();
    response.results.retain(|movie| seen_ids.insert(movie.id));
    Ok(response)
}
