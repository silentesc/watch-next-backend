use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CollectionDetailsParams {
    language: Option<String>,
}
