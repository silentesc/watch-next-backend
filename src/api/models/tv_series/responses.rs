use serde::{Deserialize, Serialize};

use crate::api::models::{configuration::responses::Language, genres::responses::Genre};

#[derive(Deserialize, Serialize)]
pub struct TvSeriesOverview {
    pub adult: Option<bool>,
    pub backdrop_path: Option<String>,
    pub poster_path: Option<String>,
    pub genre_ids: Option<Vec<u64>>,
    pub id: u64,
    pub original_language: Option<String>,
    pub original_name: Option<String>,
    pub name: Option<String>,
    pub overview: Option<String>,
    pub popularity: Option<f32>,
    pub first_air_date: Option<String>,
    pub vote_average: Option<f32>,
    pub vote_count: Option<u64>,
}

#[derive(Deserialize, Serialize)]
pub struct TvSeriesDetails {
    pub id: u64,
    pub adult: Option<bool>,
    pub softcore: Option<bool>,
    pub in_production: Option<bool>,
    pub backdrop_path: Option<String>,
    pub poster_path: Option<String>,
    pub genres: Option<Vec<Genre>>,
    pub homepage: Option<String>,
    pub origin_country: Option<Vec<String>>,
    pub original_language: Option<String>,
    pub original_name: Option<String>,
    pub name: Option<String>,
    pub overview: Option<String>,
    pub popularity: Option<f32>,
    pub production_companies: Option<Vec<ProductionCompany>>,
    pub production_countries: Option<Vec<ProductionCountry>>,
    pub networks: Option<Vec<Network>>,
    pub created_by: Option<Vec<Crew>>,
    // pub seasons: Option<?>, // TODO
    pub first_air_date: Option<String>,
    // pub last_episode_to_air: Option<?>, // TODO
    // pub next_episode_to_air: Option<?>, // TODO
    pub episode_run_time: Option<Vec<i64>>,
    pub number_of_episodes: Option<i64>,
    pub number_of_seasons: Option<i64>,
    pub languages: Option<Vec<String>>,
    pub spoken_languages: Option<Vec<Language>>,
    pub status: Option<String>,
    pub tagline: Option<String>,
    #[serde(rename = "type")]
    pub series_type: Option<String>,
    pub vote_average: Option<f32>,
    pub vote_count: Option<i64>,
}

#[derive(Deserialize, Serialize)]
pub struct ProductionCompany {
    pub id: u64,
    pub name: Option<String>,
    pub origin_country: Option<String>,
    pub logo_path: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Network {
    pub id: u64,
    pub name: Option<String>,
    pub origin_country: Option<String>,
    pub logo_path: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ProductionCountry {
    pub name: String,
    pub iso_3166_1: String,
}

#[derive(Deserialize, Serialize)]
pub struct Crew {
    pub adult: Option<bool>,
    pub gender: Option<i32>,
    pub id: u64,
    pub known_for_department: Option<String>,
    pub name: Option<String>,
    pub original_name: Option<String>,
    pub popularity: Option<f32>,
    pub profile_path: Option<String>,
    pub credit_id: Option<String>,
    pub department: Option<String>,
    pub job: Option<String>,
}
