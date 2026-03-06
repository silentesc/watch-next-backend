use axum::{Json, extract::State, http::StatusCode};
use axum_extra::extract::SignedCookieJar;

use crate::{
    api::{
        errors::AppError,
        models::auth::{LoginRequest, RegisterRequest},
        services::{self, auth},
    },
    state::AppState,
};

#[axum::debug_handler]
pub async fn register(
    State(app_state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, String), AppError> {
    match auth::register(&app_state.pool, payload.username, payload.password).await {
        Ok(_) => Ok((StatusCode::CREATED, String::from("Registered successfully"))),
        Err(app_error) => Err(app_error),
    }
}

#[axum::debug_handler]
pub async fn login(
    State(app_state): State<AppState>,
    jar: SignedCookieJar,
    Json(payload): Json<LoginRequest>,
) -> Result<(StatusCode, SignedCookieJar), AppError> {
    match services::auth::login(&app_state.pool, jar, payload.username, payload.password).await {
        Ok(signed_cookie_jar) => Ok((StatusCode::NO_CONTENT, signed_cookie_jar)),
        Err(app_error) => Err(app_error),
    }
}

#[axum::debug_handler]
pub async fn logout(
    State(app_state): State<AppState>,
    jar: SignedCookieJar,
) -> Result<(StatusCode, SignedCookieJar), AppError> {
    match services::auth::logout(&app_state.pool, jar).await {
        Ok(updated_jar) => Ok((StatusCode::OK, updated_jar)),
        Err(app_error) => Err(app_error),
    }
}
