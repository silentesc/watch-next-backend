use axum::{Extension, http::StatusCode};

use crate::api::db::models::Session;

pub async fn protected(Extension(session): Extension<Session>) -> (StatusCode, String) {
    (
        StatusCode::OK,
        format!(
            "Accessing protected endpoint with session id {} as user with id {}",
            session.id, session.user_id
        ),
    )
}
