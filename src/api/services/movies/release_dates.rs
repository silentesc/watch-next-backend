use crate::api::{
    errors::AppError,
    tmdb::{client::TmdbClient, movies::models::MovieReleaseDatesResponse},
};

pub async fn get_movie_release_dates(client: TmdbClient, movie_id: i32) -> Result<MovieReleaseDatesResponse, AppError> {
    client
        .get(format!("/movie/{movie_id}/release_dates").as_str(), &())
        .await
}
