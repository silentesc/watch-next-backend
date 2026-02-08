use axum::{Json, http::StatusCode};

use crate::api::{
    errors::AppError,
    models::auth::{LoginRequest, RegisterRequest, SessionResponse},
    services::auth::AuthService,
};

pub struct AuthHandler;

impl AuthHandler {
    pub async fn register(Json(payload): Json<RegisterRequest>) -> Result<(StatusCode, Json<SessionResponse>), AppError> {
        match AuthService::register(payload.username, payload.password).await {
            Ok(session) => Ok((StatusCode::CREATED, Json(session))),
            Err(app_error) => Err(app_error),
        }
    }

    pub async fn login(Json(payload): Json<LoginRequest>) -> Result<(StatusCode, Json<SessionResponse>), AppError> {
        match AuthService::login(payload.username, payload.password).await {
            Ok(session) => Ok((StatusCode::NO_CONTENT, Json(session))),
            Err(app_error) => Err(app_error),
        }
    }
}
