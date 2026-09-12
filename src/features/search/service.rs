use crate::{
    app::errors::AppError,
    features::search::dto::{
        SearchCollectionParams, SearchCollectionResponse, SearchMovieParams, SearchMovieResponse, SearchTvParams,
        SearchTvResponse,
    },
    integrations::tmdb::client::TmdbClient,
};

pub async fn search_collection(
    client: TmdbClient,
    params: SearchCollectionParams,
) -> Result<SearchCollectionResponse, AppError> {
    let mut response: SearchCollectionResponse = client.get("/search/collection", &params).await?;
    let mut seen_ids = std::collections::HashSet::new();
    response.results.retain(|collection| seen_ids.insert(collection.id));
    Ok(response)
}

pub async fn search_movie(client: TmdbClient, params: SearchMovieParams) -> Result<SearchMovieResponse, AppError> {
    let mut response: SearchMovieResponse = client.get("/search/movie", &params).await?;
    let mut seen_ids = std::collections::HashSet::new();
    response.results.retain(|movie| seen_ids.insert(movie.id));
    Ok(response)
}

pub async fn search_series(client: TmdbClient, params: SearchTvParams) -> Result<SearchTvResponse, AppError> {
    let mut response: SearchTvResponse = client.get("/search/tv", &params).await?;
    let mut seen_ids = std::collections::HashSet::new();
    response.results.retain(|series| seen_ids.insert(series.id));
    Ok(response)
}
