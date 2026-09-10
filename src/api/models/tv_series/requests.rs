use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TvSeriesDetailsParams {
    pub language: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct TvSeriesVideosParams {
    pub language: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct TvSeriesRecommendationsParams {
    pub language: Option<String>,
    pub page: Option<i32>,
}

#[derive(Deserialize, Serialize)]
pub struct SimilarTvSeriesParams {
    pub language: Option<String>,
    pub page: Option<i32>,
}
