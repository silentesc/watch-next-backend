use crate::api::{
    errors::AppError,
    tmdb::{client::TmdbClient, models::Language},
};

pub async fn get_languages(client: TmdbClient) -> Result<Vec<Language>, AppError> {
    client.get("/configuration/languages", &()).await
}
