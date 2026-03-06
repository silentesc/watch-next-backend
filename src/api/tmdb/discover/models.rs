use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DiscoverMovieResponseResult {
    pub adult: bool,
    pub backdrop_path: Option<String>,
    pub poster_path: Option<String>,
    pub genre_ids: Vec<u64>,
    pub id: u64,
    pub original_language: String,
    pub original_title: String,
    pub title: String,
    pub overview: String,
    pub popularity: f32,
    pub release_date: String,
    pub video: bool,
    pub vote_average: f32,
    pub vote_count: u64,
}

#[derive(Deserialize, Serialize)]
pub struct DiscoverMovieResponse {
    pub total_results: u64,
    pub total_pages: i32,
    pub page: i32,
    pub results: Vec<DiscoverMovieResponseResult>,
}

impl IntoResponse for DiscoverMovieResponse {
    fn into_response(self) -> Response {
        let body = Json(self);
        (StatusCode::OK, body).into_response()
    }
}
