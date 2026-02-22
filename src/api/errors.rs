use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

pub struct AppError {
    status_code: StatusCode,
    message: String,
}

impl AppError {
    pub fn new(status_code: StatusCode, message: String) -> Self {
        Self { status_code, message }
    }

    pub fn generic_500() -> Self {
        let status_code = StatusCode::INTERNAL_SERVER_ERROR;
        let message = String::from("An unexpected error occured");
        Self { status_code, message }
    }

    pub fn invalid_credentials() -> Self {
        let status_code = StatusCode::UNAUTHORIZED;
        let message = String::from("Invalid credentials");
        Self { status_code, message }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body = Json(json!({ "error": self.message }));
        (self.status_code, body).into_response()
    }
}
