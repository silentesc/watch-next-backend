use crate::{app::errors::AppError, features::tv_series::dto::*, integrations::tmdb::client::TmdbClient};

pub async fn get_series_details(
    client: TmdbClient,
    series_id: i32,
    params: TvSeriesDetailsParams,
) -> Result<TvSeriesDetails, AppError> {
    client
        .get(format!("/tv/{series_id}").as_str(), &params)
        .await
        .map_err(Into::into)
}

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

pub async fn get_similar_series(
    client: TmdbClient,
    series_id: i32,
    params: SimilarTvSeriesParams,
) -> Result<SimilarTvSeriesResponse, AppError> {
    let mut response: SimilarTvSeriesResponse =
        client.get(format!("/tv/{series_id}/similar").as_str(), &params).await?;
    let mut seen_ids = std::collections::HashSet::new();
    response.results.retain(|series| seen_ids.insert(series.id));
    Ok(response)
}

pub async fn get_series_videos(
    client: TmdbClient,
    series_id: i32,
    params: TvSeriesVideosParams,
) -> Result<TvSeriesVideosResponse, AppError> {
    client
        .get(format!("/tv/{series_id}/videos").as_str(), &params)
        .await
        .map_err(Into::into)
}
