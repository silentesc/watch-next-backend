use crate::{
    app::errors::AppError,
    features::discover::dto::{DiscoverMovieParams, DiscoverMovieResponse, DiscoverTvParams, DiscoverTvResponse},
    integrations::tmdb::client::TmdbClient,
};

pub async fn discover_movies(
    client: TmdbClient,
    params: DiscoverMovieParams,
) -> Result<DiscoverMovieResponse, AppError> {
    let mut response: DiscoverMovieResponse = client.get("/discover/movie", &params).await?;
    let mut seen_ids = std::collections::HashSet::new();
    response.results.retain(|movie| seen_ids.insert(movie.id));
    Ok(response)
}

pub async fn discover_tv_series(client: TmdbClient, params: DiscoverTvParams) -> Result<DiscoverTvResponse, AppError> {
    let mut response: DiscoverTvResponse = client.get("/discover/tv", &params).await?;
    let mut seen_ids = std::collections::HashSet::new();
    response.results.retain(|series| seen_ids.insert(series.id));
    Ok(response)
}
