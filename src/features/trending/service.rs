use crate::{
    app::errors::AppError,
    features::trending::dto::{
        TrendingMoviesParams, TrendingMoviesResponse, TrendingSeriesParams, TrendingSeriesResponse,
    },
    integrations::tmdb::client::TmdbClient,
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

pub async fn get_trending_series(
    client: TmdbClient,
    time_window: String,
    params: TrendingSeriesParams,
) -> Result<TrendingSeriesResponse, AppError> {
    let mut response: TrendingSeriesResponse = client
        .get(format!("/trending/tv/{time_window}").as_str(), &params)
        .await?;
    let mut seen_ids = std::collections::HashSet::new();
    response.results.retain(|series| seen_ids.insert(series.id));
    Ok(response)
}
