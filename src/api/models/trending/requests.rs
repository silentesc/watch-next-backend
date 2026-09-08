use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TrendingMoviesParams {
    pub page: Option<i32>,
    pub language: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct TrendingShowsParams {
    pub page: Option<i32>,
    pub language: Option<String>,
}
