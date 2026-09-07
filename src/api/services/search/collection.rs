use crate::api::{
    errors::AppError,
    models::search::{requests::SearchCollectionParams, responses::SearchCollectionResponse},
    tmdb::client::TmdbClient,
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
