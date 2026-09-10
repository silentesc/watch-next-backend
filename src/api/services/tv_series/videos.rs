use crate::api::{
    errors::AppError,
    models::tv_series::{requests::TvSeriesVideosParams, responses::TvSeriesVideosResponse},
    tmdb::client::TmdbClient,
};

pub async fn get_series_videos(
    client: TmdbClient,
    series_id: i32,
    params: TvSeriesVideosParams,
) -> Result<TvSeriesVideosResponse, AppError> {
    client.get(format!("/tv/{series_id}/videos").as_str(), &params).await
}
