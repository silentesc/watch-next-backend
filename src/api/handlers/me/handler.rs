use axum::{Extension, http::StatusCode};

use crate::{
    api::{
        db::models::{Session, User},
        errors::AppError,
        services,
    },
    state::AppState,
};

#[axum::debug_handler]
pub async fn me(
    Extension(app_state): Extension<AppState>,
    Extension(session): Extension<Session>,
) -> Result<(StatusCode, User), AppError> {
    match services::me::service::me(&app_state.pool, session.user_id).await {
        Ok(user) => Ok((StatusCode::OK, user)),
        Err(app_error) => Err(app_error),
    }
}
