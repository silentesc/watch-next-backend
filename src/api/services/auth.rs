use axum::http::StatusCode;
use regex::Regex;

use crate::{
    api::{errors::AppError, models::auth::SessionResponse},
    error, info,
    logger::enums::category::Category,
};

pub struct AuthService;

impl AuthService {
    pub async fn register(username: String, password: String) -> Result<SessionResponse, AppError> {
        info!(Category::Register, "Register with username {} password {}", username, password);

        // Check username length
        if username.len() < 4 || username.len() > 30 {
            return Err(AppError::new(StatusCode::BAD_REQUEST, String::from("Username length must be between 4 and 30")));
        }

        // Check username alphanumeric characters
        let re = match Regex::new(r"/^\w+$/") {
            Ok(re) => re,
            Err(err) => {
                error!(Category::Register, "Regex failed with error: {:#}", err);
                return Err(AppError::new(StatusCode::INTERNAL_SERVER_ERROR, String::from("sdf")));
            }
        };
        if !re.is_match(&username) {
            return Err(AppError::new(StatusCode::BAD_REQUEST, String::from("Username must only contain alphanumeric characters")));
        }

        // Make session
        Ok(SessionResponse { key: String::from("123456789") })
    }

    pub async fn login(username: String, password: String) -> Result<SessionResponse, AppError> {
        info!(Category::Login, "Login with username {} password {}", username, password);
        Ok(SessionResponse { key: String::from("123456789") })
    }
}
