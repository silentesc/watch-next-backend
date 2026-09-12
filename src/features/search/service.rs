use crate::{
    app::errors::AppError,
    integrations::tmdb::{
        TmdbApi,
        resources::{
            collections::dto::{SearchCollectionsParams, SearchCollectionsResponse},
            movies::dto::{SearchMoviesParams, SearchMoviesResponse},
            tv_series::dto::{SearchTvSeriesParams, SearchTvSeriesResponse},
        },
    },
};

pub async fn search_collection(
    tmdb: TmdbApi,
    params: SearchCollectionsParams,
) -> Result<SearchCollectionsResponse, AppError> {
    let mut response: SearchCollectionsResponse = tmdb.collections().search(params).await?;
    let mut seen_ids = std::collections::HashSet::new();
    response.results.retain(|collection| seen_ids.insert(collection.id));
    Ok(response)
}

pub async fn search_movie(tmdb: TmdbApi, params: SearchMoviesParams) -> Result<SearchMoviesResponse, AppError> {
    let mut response: SearchMoviesResponse = tmdb.movies().search(params).await?;
    let mut seen_ids = std::collections::HashSet::new();
    response.results.retain(|movie| seen_ids.insert(movie.id));
    Ok(response)
}

pub async fn search_series(tmdb: TmdbApi, params: SearchTvSeriesParams) -> Result<SearchTvSeriesResponse, AppError> {
    let mut response: SearchTvSeriesResponse = tmdb.tv_series().search(params).await?;
    let mut seen_ids = std::collections::HashSet::new();
    response.results.retain(|series| seen_ids.insert(series.id));
    Ok(response)
}
