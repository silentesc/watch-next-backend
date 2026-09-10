use crate::api::{
    errors::AppError,
    models::tv_series::{requests::TvSeriesDetailsParams, responses::TvSeriesDetails},
    tmdb::client::TmdbClient,
};

pub async fn get_series_details(
    client: TmdbClient,
    series_id: i32,
    params: TvSeriesDetailsParams,
) -> Result<TvSeriesDetails, AppError> {
    client.get(format!("/tv/{}", series_id).as_str(), &params).await
}
