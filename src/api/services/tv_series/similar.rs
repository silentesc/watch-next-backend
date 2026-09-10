use crate::api::{
    errors::AppError,
    models::tv_series::{requests::SimilarTvSeriesParams, responses::SimilarTvSeriesResponse},
    tmdb::client::TmdbClient,
};

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
