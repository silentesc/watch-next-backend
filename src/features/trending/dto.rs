use crate::features::{movies::dto::MovieOverview, tv_series::dto::TvSeriesOverview};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TrendingMoviesParams {
    pub page: Option<i32>,
    pub language: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct TrendingSeriesParams {
    pub page: Option<i32>,
    pub language: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct TrendingMoviesResponse {
    pub total_results: u64,
    pub total_pages: i32,
    pub page: i32,
    pub results: Vec<MovieOverview>,
}

#[derive(Deserialize, Serialize)]
pub struct TrendingSeriesResponse {
    pub total_results: u64,
    pub total_pages: i32,
    pub page: i32,
    pub results: Vec<TvSeriesOverview>,
}
