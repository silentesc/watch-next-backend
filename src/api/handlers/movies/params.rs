use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct MovieDetailsParams {
    #[serde(rename = "language")]
    language: Option<String>,
}
