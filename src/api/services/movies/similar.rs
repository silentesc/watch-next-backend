use crate::api::{
    errors::AppError,
    models::movies::{requests::SimilarMoviesParams, responses::SimilarMoviesResponse},
    tmdb::client::TmdbClient,
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
