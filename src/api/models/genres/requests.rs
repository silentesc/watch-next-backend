use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GenreMovieParams {
    pub language: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct GenreTvParams {
    pub language: Option<String>,
}
