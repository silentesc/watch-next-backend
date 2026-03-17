use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GenreMovieParams {
    #[serde(rename = "language")]
    language: Option<String>,
}
