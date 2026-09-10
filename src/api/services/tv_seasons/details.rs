use crate::api::{
    errors::AppError,
    models::tv_seasons::{requests::TvSeasonDetailsParams, responses::TvSeasonDetails},
    tmdb::client::TmdbClient,
};

pub async fn get_season_details(
    client: TmdbClient,
    series_id: i32,
    season_id: i32,
    params: TvSeasonDetailsParams,
) -> Result<TvSeasonDetails, AppError> {
    client
        .get(format!("/tv/{}/season/{}", series_id, season_id).as_str(), &params)
        .await
}
