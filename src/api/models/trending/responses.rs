use serde::{Deserialize, Serialize};

use crate::api::models::movies::responses::MovieOverview;

#[derive(Deserialize, Serialize)]
pub struct TrendingMoviesResponse {
    pub total_results: u64,
    pub total_pages: i32,
    pub page: i32,
    pub results: Vec<MovieOverview>,
}
