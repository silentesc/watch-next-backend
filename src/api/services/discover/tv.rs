use crate::api::{
    errors::AppError,
    models::discover::{requests::DiscoverTvParams, responses::DiscoverTvResponse},
    tmdb::client::TmdbClient,
};

pub async fn discover(client: TmdbClient, params: DiscoverTvParams) -> Result<DiscoverTvResponse, AppError> {
    let mut response: DiscoverTvResponse = client.get("/discover/tv", &params).await?;

    let mut seen_ids = std::collections::HashSet::new();
    response.results.retain(|series| seen_ids.insert(series.id));
    Ok(response)
}
