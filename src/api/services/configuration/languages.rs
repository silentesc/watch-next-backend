use reqwest::Client;

use crate::api::{
    errors::AppError,
    tmdb::{self, models::Language},
};

pub async fn get_languages(client: Client) -> Result<Vec<Language>, AppError> {
    match tmdb::configuration::languages::get_languages(client).await {
        Ok(response) => Ok(response),
        Err(app_error) => Err(app_error),
    }
}
