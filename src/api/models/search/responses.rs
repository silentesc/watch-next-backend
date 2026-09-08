use serde::{Deserialize, Serialize};

use crate::api::models::{
    collections::responses::CollectionOverview, movies::responses::MovieOverview, tv::responses::TvOverview,
};

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
    pub results: Vec<TvOverview>,
}

#[derive(Deserialize, Serialize)]
pub struct SearchCollectionResponse {
    pub total_results: u64,
    pub total_pages: i32,
    pub page: i32,
    pub results: Vec<CollectionOverview>,
}
