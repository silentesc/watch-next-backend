use axum::http::StatusCode;

pub struct RootHandler;

impl RootHandler {
    pub async fn root() -> (StatusCode, &'static str) {
        (StatusCode::OK, "Up and running!")
    }
}
