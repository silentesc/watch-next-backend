use crate::{
    app::errors::AppError,
    integrations::tmdb::{
        TmdbApi,
        models::tv_series::TvSeriesDetails,
        resources::tv_series::dto::{
            SimilarTvSeriesParams, SimilarTvSeriesResponse, TvSeriesDetailsParams, TvSeriesRecommendationsParams,
            TvSeriesRecommendationsResponse, TvSeriesVideosParams, TvSeriesVideosResponse,
        },
    },
};

pub async fn get_series_details(
    tmdb: TmdbApi,
    series_id: i32,
    params: TvSeriesDetailsParams,
) -> Result<TvSeriesDetails, AppError> {
    tmdb.tv_series().details(series_id, params).await.map_err(Into::into)
}

pub async fn get_series_recommendations(
    tmdb: TmdbApi,
    series_id: i32,
    params: TvSeriesRecommendationsParams,
) -> Result<TvSeriesRecommendationsResponse, AppError> {
    let mut response: TvSeriesRecommendationsResponse = tmdb.tv_series().recommendations(series_id, params).await?;
    let mut seen_ids = std::collections::HashSet::new();
    response.results.retain(|series| seen_ids.insert(series.id));
    Ok(response)
}

pub async fn get_similar_series(
    tmdb: TmdbApi,
    series_id: i32,
    params: SimilarTvSeriesParams,
) -> Result<SimilarTvSeriesResponse, AppError> {
    let mut response: SimilarTvSeriesResponse = tmdb.tv_series().similar(series_id, params).await?;
    let mut seen_ids = std::collections::HashSet::new();
    response.results.retain(|series| seen_ids.insert(series.id));
    Ok(response)
}

pub async fn get_series_videos(
    tmdb: TmdbApi,
    series_id: i32,
    params: TvSeriesVideosParams,
) -> Result<TvSeriesVideosResponse, AppError> {
    tmdb.tv_series().videos(series_id, params).await.map_err(Into::into)
}
