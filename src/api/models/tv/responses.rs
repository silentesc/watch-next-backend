use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TvOverview {
    pub adult: Option<bool>,
    pub backdrop_path: Option<String>,
    pub poster_path: Option<String>,
    pub genre_ids: Option<Vec<u64>>,
    pub id: u64,
    pub original_language: Option<String>,
    pub original_name: Option<String>,
    pub name: Option<String>,
    pub overview: Option<String>,
    pub popularity: Option<f32>,
    pub first_air_date: Option<String>,
    pub vote_average: Option<f32>,
    pub vote_count: Option<u64>,
}
