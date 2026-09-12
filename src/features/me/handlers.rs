use axum::{Extension, Json, http::StatusCode};

use crate::{
    app::{errors::AppError, state::AppState},
    persistence::models::{Session, User},
};

#[axum::debug_handler]
pub async fn me(
    Extension(app_state): Extension<AppState>,
    Extension(session): Extension<Session>,
) -> Result<(StatusCode, Json<User>), AppError> {
    match crate::features::me::service::me(&app_state.pool, session.user_id).await {
        Ok(user) => Ok((StatusCode::OK, Json(user))),
        Err(app_error) => Err(app_error),
    }
}
