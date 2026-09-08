use crate::api::{
    errors::AppError,
    models::trending::{requests::TrendingShowsParams, responses::TrendingShowsResponse},
    tmdb::client::TmdbClient,
};

pub async fn get_trending_shows(
    client: TmdbClient,
    time_window: String,
    params: TrendingShowsParams,
) -> Result<TrendingShowsResponse, AppError> {
    let mut response: TrendingShowsResponse = client
        .get(format!("/trending/tv/{time_window}").as_str(), &params)
        .await?;

    let mut seen_ids = std::collections::HashSet::new();
    response.results.retain(|show| seen_ids.insert(show.id));
    Ok(response)
}
