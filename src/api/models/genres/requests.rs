use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GenreMovieParams {
    pub language: Option<String>,
}
