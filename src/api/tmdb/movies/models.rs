use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

use crate::api::tmdb::models::ReleaseDates;

#[derive(Deserialize, Serialize)]
pub struct MovieReleaseDatesResponse {
    pub id: u64,
    pub results: Vec<ReleaseDates>,
}

impl IntoResponse for MovieReleaseDatesResponse {
    fn into_response(self) -> Response {
        let body = Json(self);
        (StatusCode::OK, body).into_response()
    }
}
