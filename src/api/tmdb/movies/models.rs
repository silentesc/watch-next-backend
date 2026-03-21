use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

use crate::api::tmdb::models::{Cast, Crew, ReleaseDates, Video};

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

#[derive(Deserialize, Serialize)]
pub struct MovieCreditsResponse {
    pub id: u64,
    pub cast: Vec<Cast>,
    pub crew: Vec<Crew>,
}

impl IntoResponse for MovieCreditsResponse {
    fn into_response(self) -> Response {
        let body = Json(self);
        (StatusCode::OK, body).into_response()
    }
}

#[derive(Deserialize, Serialize)]
pub struct MovieVideosResponse {
    pub id: u64,
    pub results: Vec<Video>,
}

impl IntoResponse for MovieVideosResponse {
    fn into_response(self) -> Response {
        let body = Json(self);
        (StatusCode::OK, body).into_response()
    }
}
