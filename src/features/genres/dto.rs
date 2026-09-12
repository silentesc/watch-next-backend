use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GenreMovieParams {
    pub language: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct GenreTvParams {
    pub language: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Genre {
    pub id: u64,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct GenreMovieResponse {
    pub genres: Vec<Genre>,
}

#[derive(Deserialize, Serialize)]
pub struct GenreTvResponse {
    pub genres: Vec<Genre>,
}
