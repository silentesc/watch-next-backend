use crate::{app::errors::AppError, features::configuration::dto::Language, integrations::tmdb::client::TmdbClient};

pub async fn get_languages(client: TmdbClient) -> Result<Vec<Language>, AppError> {
    client.get("/configuration/languages", &()).await
}
