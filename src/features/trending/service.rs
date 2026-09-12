use crate::{
    app::errors::AppError,
    integrations::tmdb::{
        TmdbApi,
        models::common::TimeWindow,
        resources::{
            movies::dto::{TrendingMoviesParams, TrendingMoviesResponse},
            tv_series::dto::{TrendingTvSeriesParams, TrendingTvSeriesResponse},
        },
    },
};

pub async fn get_trending_movies(
    tmdb: TmdbApi,
    time_window: String,
    params: TrendingMoviesParams,
) -> Result<TrendingMoviesResponse, AppError> {
    let mut response: TrendingMoviesResponse = tmdb
        .movies()
        .trending(TimeWindow::from_str(&time_window), params)
        .await?;
    let mut seen_ids = std::collections::HashSet::new();
    response.results.retain(|movie| seen_ids.insert(movie.id));
    Ok(response)
}

pub async fn get_trending_series(
    tmdb: TmdbApi,
    time_window: String,
    params: TrendingTvSeriesParams,
) -> Result<TrendingTvSeriesResponse, AppError> {
    let mut response: TrendingTvSeriesResponse = tmdb
        .tv_series()
        .trending(TimeWindow::from_str(&time_window), params)
        .await?;
    let mut seen_ids = std::collections::HashSet::new();
    response.results.retain(|series| seen_ids.insert(series.id));
    Ok(response)
}
