use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Language {
    pub iso_639_1: String,
    pub english_name: String,
    pub name: String,
}

impl IntoResponse for Language {
    fn into_response(self) -> Response {
        let body = Json(self);
        (StatusCode::OK, body).into_response()
    }
}
