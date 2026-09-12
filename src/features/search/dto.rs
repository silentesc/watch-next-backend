use crate::features::{collections::dto::CollectionOverview, movies::dto::MovieOverview, tv_series::dto::TvSeriesOverview};
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
pub struct SearchTvParams {
    pub query: String,
    pub page: Option<i32>,
    pub include_adult: Option<bool>,
    pub language: Option<String>,
    pub first_air_date_year: Option<i32>,
    pub year: Option<i32>,
}

#[derive(Deserialize, Serialize)]
pub struct SearchCollectionParams {
    pub query: String,
    pub page: Option<i32>,
    pub include_adult: Option<bool>,
    pub language: Option<String>,
    pub region: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct SearchMovieResponse {
    pub total_results: u64,
    pub total_pages: i32,
    pub page: i32,
    pub results: Vec<MovieOverview>,
}

#[derive(Deserialize, Serialize)]
pub struct SearchTvResponse {
    pub total_results: u64,
    pub total_pages: i32,
    pub page: i32,
    pub results: Vec<TvSeriesOverview>,
}

#[derive(Deserialize, Serialize)]
pub struct SearchCollectionResponse {
    pub total_results: u64,
    pub total_pages: i32,
    pub page: i32,
    pub results: Vec<CollectionOverview>,
}
