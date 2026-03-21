use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct MovieDetailsParams {
    language: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct MovieCreditsParams {
    language: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct MovieVideosParams {
    language: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct MovieRecommendationsParams {
    language: Option<String>,
    page: Option<i32>,
}
