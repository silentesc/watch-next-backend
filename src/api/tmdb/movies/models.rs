use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

use crate::api::tmdb::models::{Cast, Crew, MovieOverview, ReleaseDates, Video};

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

#[derive(Deserialize, Serialize)]
pub struct MovieRecommendationsResponse {
    pub total_results: u64,
    pub total_pages: i32,
    pub page: i32,
    pub results: Vec<MovieOverview>,
}

impl IntoResponse for MovieRecommendationsResponse {
    fn into_response(self) -> Response {
        let body = Json(self);
        (StatusCode::OK, body).into_response()
    }
}

#[derive(Deserialize, Serialize)]
pub struct SimilarMoviesResponse {
    pub total_results: u64,
    pub total_pages: i32,
    pub page: i32,
    pub results: Vec<MovieOverview>,
}

impl IntoResponse for SimilarMoviesResponse {
    fn into_response(self) -> Response {
        let body = Json(self);
        (StatusCode::OK, body).into_response()
    }
}
