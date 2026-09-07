use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
use reqwest::Client;
use sqlx::PgPool;

use crate::api::tmdb::client::TmdbClient;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub client: Client,
    pub tmdb_client: TmdbClient,
    pub key: Key,
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}
