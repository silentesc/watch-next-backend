use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct MovieDetailsParams {
    pub language: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct MovieCreditsParams {
    pub language: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct MovieVideosParams {
    pub language: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct MovieRecommendationsParams {
    pub language: Option<String>,
    pub page: Option<i32>,
}

#[derive(Deserialize, Serialize)]
pub struct SimilarMoviesParams {
    pub language: Option<String>,
    pub page: Option<i32>,
}
