use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
use sqlx::PgPool;

use crate::integrations::tmdb::client::TmdbClient;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub tmdb_client: TmdbClient,
    pub key: Key,
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
}
