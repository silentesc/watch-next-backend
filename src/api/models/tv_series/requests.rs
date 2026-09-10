use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TvSeriesDetailsParams {
    pub language: Option<String>,
}
