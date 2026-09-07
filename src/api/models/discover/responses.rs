use serde::{Deserialize, Serialize};

use crate::api::models::{movies::responses::MovieOverview, tv::responses::TvOverview};

#[derive(Deserialize, Serialize)]
pub struct DiscoverMovieResponse {
    pub total_results: u64,
    pub total_pages: i32,
    pub page: i32,
    pub results: Vec<MovieOverview>,
}

#[derive(Deserialize, Serialize)]
pub struct DiscoverTvResponse {
    pub total_results: u64,
    pub total_pages: i32,
    pub page: i32,
    pub results: Vec<TvOverview>,
}
