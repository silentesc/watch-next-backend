use crate::api::{
    errors::AppError,
    models::search::{requests::SearchTvParams, responses::SearchTvResponse},
    tmdb::client::TmdbClient,
};

pub async fn search_series(client: TmdbClient, params: SearchTvParams) -> Result<SearchTvResponse, AppError> {
    let mut response: SearchTvResponse = client.get("/search/tv", &params).await?;

    let mut seen_ids = std::collections::HashSet::new();
    response.results.retain(|series| seen_ids.insert(series.id));
    Ok(response)
}
