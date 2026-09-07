use crate::api::{errors::AppError, models::configuration::responses::Language, tmdb::client::TmdbClient};

pub async fn get_languages(client: TmdbClient) -> Result<Vec<Language>, AppError> {
    client.get("/configuration/languages", &()).await
}
