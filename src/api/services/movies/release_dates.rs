use crate::api::{errors::AppError, models::movies::responses::MovieReleaseDatesResponse, tmdb::client::TmdbClient};

pub async fn get_movie_release_dates(client: TmdbClient, movie_id: i32) -> Result<MovieReleaseDatesResponse, AppError> {
    client
        .get(format!("/movie/{movie_id}/release_dates").as_str(), &())
        .await
}
