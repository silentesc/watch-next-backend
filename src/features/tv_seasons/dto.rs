use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TvSeasonDetailsParams {
    pub language: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct TvSeasonOverview {
    pub air_date: Option<String>,
    pub episode_count: Option<i64>,
    pub id: u64,
    pub name: Option<String>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub season_number: Option<i64>,
    pub vote_average: Option<f32>,
}

#[derive(Deserialize, Serialize)]
pub struct TvSeasonDetails {
    pub air_date: Option<String>,
    pub episodes: Option<Vec<TvEpisode>>,
    pub name: Option<String>,
    pub networks: Option<Vec<TvSeasonNetwork>>,
    pub overview: Option<String>,
    pub id: u64,
    pub poster_path: Option<String>,
    pub season_number: Option<i64>,
    pub vote_average: Option<f32>,
}

#[derive(Deserialize, Serialize)]
pub struct TvEpisode {
    pub air_date: Option<String>,
    pub episode_number: Option<i64>,
    pub episode_type: Option<String>,
    pub id: u64,
    pub name: Option<String>,
    pub overview: Option<String>,
    pub production_code: Option<String>,
    pub runtime: Option<i64>,
    pub season_number: Option<i64>,
    pub show_id: Option<u64>,
    pub still_path: Option<String>,
    pub vote_average: Option<f32>,
    pub vote_count: Option<i64>,
    pub crew: Option<Vec<TvEpisodeCrew>>,
    pub guest_stars: Option<Vec<TvEpisodeGuestStar>>,
}

#[derive(Deserialize, Serialize)]
pub struct TvEpisodeCrew {
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

#[derive(Deserialize, Serialize)]
pub struct TvEpisodeGuestStar {
    pub character: Option<String>,
    pub credit_id: Option<String>,
    pub order: Option<i64>,
    pub adult: Option<bool>,
    pub gender: Option<i32>,
    pub id: u64,
    pub known_for_department: Option<String>,
    pub name: Option<String>,
    pub original_name: Option<String>,
    pub popularity: Option<f32>,
    pub profile_path: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct TvSeasonNetwork {
    pub id: u64,
    pub name: Option<String>,
    pub origin_country: Option<String>,
    pub logo_path: Option<String>,
}
