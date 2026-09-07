use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Language {
    pub iso_639_1: String,
    pub english_name: String,
    pub name: String,
}
