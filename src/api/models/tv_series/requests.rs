use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TvSeriesDetailsParams {
    pub language: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct TvSeriesVideosParams {
    pub language: Option<String>,
}
