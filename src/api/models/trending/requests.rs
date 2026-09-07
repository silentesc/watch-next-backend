use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TrendingMoviesParams {
    pub page: Option<i32>,
    pub language: Option<String>,
}
