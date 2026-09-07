use crate::api::{
    errors::AppError,
    models::trending::{requests::TrendingMoviesParams, responses::TrendingMoviesResponse},
    tmdb::client::TmdbClient,
};

pub async fn get_trending_movies(
    client: TmdbClient,
    time_window: String,
    params: TrendingMoviesParams,
) -> Result<TrendingMoviesResponse, AppError> {
    let mut response: TrendingMoviesResponse = client
        .get(format!("/trending/movie/{time_window}").as_str(), &params)
        .await?;

    let mut seen_ids = std::collections::HashSet::new();
    response.results.retain(|movie| seen_ids.insert(movie.id));
    Ok(response)
}
