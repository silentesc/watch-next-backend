use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CollectionDetailsParams {
    pub language: Option<String>,
}
