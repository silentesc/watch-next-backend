use crate::api::{
    errors::AppError,
    models::tv_series::{requests::TvSeriesRecommendationsParams, responses::TvSeriesRecommendationsResponse},
    tmdb::client::TmdbClient,
};

pub async fn get_series_recommendations(
    client: TmdbClient,
    series_id: i32,
    params: TvSeriesRecommendationsParams,
) -> Result<TvSeriesRecommendationsResponse, AppError> {
    let mut response: TvSeriesRecommendationsResponse = client
        .get(format!("/tv/{series_id}/recommendations").as_str(), &params)
        .await?;

    let mut seen_ids = std::collections::HashSet::new();
    response.results.retain(|series| seen_ids.insert(series.id));
    Ok(response)
}
