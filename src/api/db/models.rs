use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::api::utils::time_utils;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub last_login_at: Option<NaiveDateTime>,
}

impl IntoResponse for User {
    fn into_response(self) -> Response {
        let body = Json(json!({ "username": self.username }));
        (StatusCode::OK, body).into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub expires_at: NaiveDateTime,
}

impl Session {
    pub fn is_expired(&self) -> bool {
        self.expires_at < time_utils::utc_now_naive()
    }
}
