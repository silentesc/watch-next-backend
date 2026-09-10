use crate::api::{
    errors::AppError,
    models::trending::{requests::TrendingSeriesParams, responses::TrendingSeriesResponse},
    tmdb::client::TmdbClient,
};

pub async fn get_trending_series(
    client: TmdbClient,
    time_window: String,
    params: TrendingSeriesParams,
) -> Result<TrendingSeriesResponse, AppError> {
    let mut response: TrendingSeriesResponse = client
        .get(format!("/trending/tv/{time_window}").as_str(), &params)
        .await?;

    let mut seen_ids = std::collections::HashSet::new();
    response.results.retain(|series| seen_ids.insert(series.id));
    Ok(response)
}
