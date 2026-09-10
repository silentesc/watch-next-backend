use crate::api::{
    errors::AppError,
    models::tv::{requests::TvDetailsParams, responses::TvDetails},
    tmdb::client::TmdbClient,
};

pub async fn get_series_details(
    client: TmdbClient,
    series_id: i32,
    params: TvDetailsParams,
) -> Result<TvDetails, AppError> {
    client.get(format!("/tv/{}", series_id).as_str(), &params).await
}
