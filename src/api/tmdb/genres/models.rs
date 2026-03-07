use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Genre {
    pub id: u64,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct GenreMovieListResponse {
    pub genres: Vec<Genre>,
}

impl IntoResponse for GenreMovieListResponse {
    fn into_response(self) -> Response {
        let body = Json(self);
        (StatusCode::OK, body).into_response()
    }
}
