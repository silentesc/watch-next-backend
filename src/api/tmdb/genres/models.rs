use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

use crate::api::tmdb::models::Genre;

#[derive(Deserialize, Serialize)]
pub struct GenreMovieResponse {
    pub genres: Vec<Genre>,
}

impl IntoResponse for GenreMovieResponse {
    fn into_response(self) -> Response {
        let body = Json(self);
        (StatusCode::OK, body).into_response()
    }
}
