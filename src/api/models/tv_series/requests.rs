use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TvDetailsParams {
    pub language: Option<String>,
}
