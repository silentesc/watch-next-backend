use crate::features::{
    configuration::dto::Language,
    genres::dto::Genre,
    tv_seasons::dto::{TvEpisode, TvSeasonOverview},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TvSeriesDetailsParams {
    pub language: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct TvSeriesVideosParams {
    pub language: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct TvSeriesRecommendationsParams {
    pub language: Option<String>,
    pub page: Option<i32>,
}

#[derive(Deserialize, Serialize)]
pub struct SimilarTvSeriesParams {
    pub language: Option<String>,
    pub page: Option<i32>,
}

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
    pub production_companies: Option<Vec<TvSeriesProductionCompany>>,
    pub production_countries: Option<Vec<TvSeriesProductionCountry>>,
    pub networks: Option<Vec<TvSeriesNetwork>>,
    pub created_by: Option<Vec<TvSeriesCreators>>,
    pub seasons: Option<Vec<TvSeasonOverview>>,
    pub first_air_date: Option<String>,
    pub last_episode_to_air: Option<TvEpisode>,
    pub next_episode_to_air: Option<TvEpisode>,
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
pub struct TvSeriesProductionCompany {
    pub id: u64,
    pub name: Option<String>,
    pub origin_country: Option<String>,
    pub logo_path: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct TvSeriesNetwork {
    pub id: u64,
    pub name: Option<String>,
    pub origin_country: Option<String>,
    pub logo_path: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct TvSeriesProductionCountry {
    pub name: String,
    pub iso_3166_1: String,
}

#[derive(Deserialize, Serialize)]
pub struct TvSeriesCreators {
    pub id: u64,
    pub credit_id: Option<String>,
    pub name: Option<String>,
    pub gender: Option<i32>,
    pub profile_path: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct TvSeriesVideo {
    #[serde(rename = "iso_639_1")]
    pub language: Option<String>,
    #[serde(rename = "iso_3166_1")]
    pub name: Option<String>,
    pub key: Option<String>,
    pub site: Option<String>,
    pub size: Option<i32>,
    #[serde(rename = "type")]
    pub video_type: Option<String>,
    pub official: Option<bool>,
    pub published_at: Option<String>,
    pub id: String,
}

#[derive(Deserialize, Serialize)]
pub struct TvSeriesVideosResponse {
    pub id: u64,
    pub results: Vec<TvSeriesVideo>,
}

#[derive(Deserialize, Serialize)]
pub struct TvSeriesRecommendationsResponse {
    pub total_results: u64,
    pub total_pages: i32,
    pub page: i32,
    pub results: Vec<TvSeriesOverview>,
}

#[derive(Deserialize, Serialize)]
pub struct SimilarTvSeriesResponse {
    pub total_results: u64,
    pub total_pages: i32,
    pub page: i32,
    pub results: Vec<TvSeriesOverview>,
}
