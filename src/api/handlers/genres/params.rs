use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct GenreMovieListParams {
    #[serde(rename = "language")]
    language: Option<String>,
}
