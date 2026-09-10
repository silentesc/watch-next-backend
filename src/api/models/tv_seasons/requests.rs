use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TvSeasonDetailsParams {
    pub language: Option<String>,
}
