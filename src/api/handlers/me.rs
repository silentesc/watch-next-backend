use axum::Extension;

use crate::{
    api::{
        db::models::{Session, User},
        errors::AppError,
        services,
    },
    state::AppState,
};

pub async fn me(
    Extension(app_state): Extension<AppState>,
    Extension(session): Extension<Session>,
) -> Result<User, AppError> {
    match services::me::me(&app_state.pool, session.user_id).await {
        Ok(user) => Ok(user),
        Err(app_error) => Err(app_error),
    }
}
