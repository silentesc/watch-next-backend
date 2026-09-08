use crate::api::{
    errors::AppError,
    models::genres::{requests::GenreTvParams, responses::GenreTvResponse},
    tmdb::client::TmdbClient,
};

pub async fn get_tv_genres(client: TmdbClient, params: GenreTvParams) -> Result<GenreTvResponse, AppError> {
    client.get("/genre/tv/list", &params).await
}
