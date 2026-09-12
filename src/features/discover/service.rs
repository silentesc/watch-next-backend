use crate::{
    app::errors::AppError,
    integrations::tmdb::{
        TmdbApi,
        resources::{
            movies::dto::{DiscoverMovieParams, DiscoverMovieResponse},
            tv_series::dto::{DiscoverTvSeriesParams, DiscoverTvSeriesResponse},
        },
    },
};

pub async fn discover_movies(tmdb: TmdbApi, params: DiscoverMovieParams) -> Result<DiscoverMovieResponse, AppError> {
    let mut response: DiscoverMovieResponse = tmdb.movies().discover(params).await?;
    let mut seen_ids = std::collections::HashSet::new();
    response.results.retain(|movie| seen_ids.insert(movie.id));
    Ok(response)
}

pub async fn discover_tv_series(
    tmdb: TmdbApi,
    params: DiscoverTvSeriesParams,
) -> Result<DiscoverTvSeriesResponse, AppError> {
    let mut response: DiscoverTvSeriesResponse = tmdb.tv_series().discover(params).await?;
    let mut seen_ids = std::collections::HashSet::new();
    response.results.retain(|series| seen_ids.insert(series.id));
    Ok(response)
}
