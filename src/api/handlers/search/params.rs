use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SearchMovieParams {
    query: String,
    page: Option<i32>,
    include_adult: Option<bool>,
    language: Option<String>,
    primary_release_year: Option<String>,
    region: Option<String>,
    year: Option<String>,
}
