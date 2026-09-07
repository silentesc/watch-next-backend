use crate::api::{
    errors::AppError,
    models::movies::{requests::MovieCreditsParams, responses::MovieCreditsResponse},
    tmdb::client::TmdbClient,
};

pub async fn get_movie_credits(
    client: TmdbClient,
    movie_id: i32,
    params: MovieCreditsParams,
) -> Result<MovieCreditsResponse, AppError> {
    client.get(format!("/movie/{movie_id}/credits").as_str(), &params).await
}
