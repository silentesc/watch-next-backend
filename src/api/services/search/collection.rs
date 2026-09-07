use crate::api::{
    errors::AppError,
    handlers::search::params::SearchCollectionParams,
    tmdb::{client::TmdbClient, search::models::SearchCollectionResponse},
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
