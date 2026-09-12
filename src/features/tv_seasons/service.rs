use crate::{
    app::errors::AppError,
    features::tv_seasons::dto::{TvSeasonDetails, TvSeasonDetailsParams},
    integrations::tmdb::client::TmdbClient,
};

pub async fn get_season_details(
    client: TmdbClient,
    series_id: i32,
    season_id: i32,
    params: TvSeasonDetailsParams,
) -> Result<TvSeasonDetails, AppError> {
    client
        .get(&format!("/tv/{series_id}/season/{season_id}"), &params)
        .await
        .map_err(Into::into)
}
