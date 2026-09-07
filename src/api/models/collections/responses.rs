use serde::{Deserialize, Serialize};

use crate::api::models::movies::responses::MovieOverview;

#[derive(Deserialize, Serialize)]
pub struct CollectionOverview {
    pub id: u64,
    pub name: String,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub adult: Option<bool>,
    pub original_language: Option<String>,
    pub original_name: Option<String>,
    pub overview: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct CollectionDetails {
    pub id: u64,
    pub name: Option<String>,
    pub original_language: Option<String>,
    pub original_name: Option<String>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub parts: Vec<MovieOverview>,
}
