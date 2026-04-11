use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TrendingMoviesParams {
    page: Option<i32>,
    language: Option<String>,
}
