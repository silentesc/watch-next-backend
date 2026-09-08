use crate::api::{
    errors::AppError,
    models::tv::{requests::TvDetailsParams, responses::TvDetails},
    tmdb::client::TmdbClient,
};

pub async fn get_show_details(
    client: TmdbClient,
    show_id: i32,
    params: TvDetailsParams,
) -> Result<TvDetails, AppError> {
    client.get(format!("/tv/{}", show_id).as_str(), &params).await
}
