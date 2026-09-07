use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SearchMovieParams {
    pub query: String,
    pub page: Option<i32>,
    pub include_adult: Option<bool>,
    pub language: Option<String>,
    pub primary_release_year: Option<String>,
    pub region: Option<String>,
    pub year: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct SearchCollectionParams {
    pub query: String,
    pub page: Option<i32>,
    pub include_adult: Option<bool>,
    pub language: Option<String>,
    pub region: Option<String>,
}
