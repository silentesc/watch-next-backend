use axum::http::StatusCode;
use axum_extra::extract::SignedCookieJar;
use regex::Regex;
use sqlx::PgPool;

use crate::{
    api::{
        db::table_utils::{sessions, users},
        errors::AppError,
        utils::cookie_utils,
    },
    error, info,
    logger::enums::category::Category,
};

pub struct AuthService;

impl AuthService {
    pub async fn register(pool: &PgPool, username: String, password: String) -> Result<(), AppError> {
        // Check username length
        if username.len() < 4 || username.len() > 30 {
            return Err(AppError::new(
                StatusCode::BAD_REQUEST,
                String::from("Username length must be between 4 and 30"),
            ));
        }

        // Check username alphanumeric characters
        let re = match Regex::new(r"^\w+$") {
            Ok(re) => re,
            Err(err) => {
                error!(Category::Register, "Regex failed with error: {:#}", err);
                return Err(AppError::generic_500());
            }
        };
        if !re.is_match(&username) {
            return Err(AppError::new(
                StatusCode::BAD_REQUEST,
                String::from("Username must only contain alphanumeric characters"),
            ));
        }

        // Check if username is already used
        match users::get_user_by_username(pool, &username).await {
            Ok(user) => {
                if user.is_some() {
                    return Err(AppError::new(
                        StatusCode::BAD_REQUEST,
                        String::from("Username is already taken"),
                    ));
                }
            }
            Err(app_error) => return Err(app_error),
        };

        // Hash password
        let password_hashed = match bcrypt::hash(password, bcrypt::DEFAULT_COST) {
            Ok(password_hashed) => password_hashed,
            Err(err) => {
                error!(Category::Register, "Bcrypt hash failed with error: {:#}", err);
                return Err(AppError::generic_500());
            }
        };

        // Create user in db
        match users::create_user(pool, &username, &password_hashed).await {
            Ok(()) => {
                info!(Category::Register, "User registered: {}", username);
                Ok(())
            }
            Err(app_error) => Err(app_error),
        }
    }

    pub async fn login(
        pool: &PgPool,
        jar: SignedCookieJar,
        username: String,
        password: String,
    ) -> Result<SignedCookieJar, AppError> {
        // Get user by username
        let db_user = match users::get_user_by_username(pool, &username).await {
            Ok(db_user) => db_user,
            Err(app_error) => return Err(app_error),
        };

        // Check if user exists
        let db_user = match db_user {
            Some(db_user) => db_user,
            None => return Err(AppError::invalid_credentials()),
        };

        // Check if user password matches
        let verified = match bcrypt::verify(&password, &db_user.password_hash) {
            Ok(verified) => verified,
            Err(err) => {
                error!(
                    Category::Login,
                    "Verifying password with bcrypt failed with error: {:#}", err
                );
                return Err(AppError::generic_500());
            }
        };
        if !verified {
            return Err(AppError::invalid_credentials());
        }

        // Create session
        let session_id = match sessions::create_session(pool, db_user.id, cookie_utils::expires_at_naive()).await {
            Ok(session_id) => session_id,
            Err(app_error) => return Err(app_error),
        };

        // Create cookie
        let cookie = cookie_utils::default_cookie(session_id.to_string(), cookie_utils::expires_at_offset());
        let signed_cookie_jar = jar.add(cookie);

        // Set last login
        match users::update_last_login_to_now(pool, &db_user.username).await {
            Ok(_) => info!(Category::Login, "User logged in: {}", username),
            Err(app_error) => return Err(app_error),
        };

        Ok(signed_cookie_jar)
    }
}
