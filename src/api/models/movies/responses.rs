use serde::{Deserialize, Serialize};

use crate::api::models::{
    collections::responses::CollectionOverview, configuration::responses::Language, genres::responses::Genre,
};

#[derive(Deserialize, Serialize)]
pub struct MovieOverview {
    pub adult: Option<bool>,
    pub backdrop_path: Option<String>,
    pub poster_path: Option<String>,
    pub genre_ids: Option<Vec<u64>>,
    pub id: u64,
    pub original_language: Option<String>,
    pub original_title: Option<String>,
    pub title: Option<String>,
    pub overview: Option<String>,
    pub popularity: Option<f32>,
    pub release_date: Option<String>,
    pub video: Option<bool>,
    pub vote_average: Option<f32>,
    pub vote_count: Option<u64>,
}

#[derive(Deserialize, Serialize)]
pub struct MovieDetails {
    pub id: u64,
    pub imdb_id: Option<String>,
    pub adult: Option<bool>,
    pub backdrop_path: Option<String>,
    pub poster_path: Option<String>,
    pub belongs_to_collection: Option<CollectionOverview>,
    pub budget: Option<i64>,
    pub genres: Option<Vec<Genre>>,
    pub homepage: Option<String>,
    pub origin_country: Option<Vec<String>>,
    pub original_language: Option<String>,
    pub original_title: Option<String>,
    pub overview: Option<String>,
    pub popularity: Option<f32>,
    pub production_companies: Option<Vec<ProductionCompany>>,
    pub production_countries: Option<Vec<ProductionCountry>>,
    pub release_date: Option<String>,
    pub revenue: Option<i64>,
    pub runtime: Option<i64>,
    pub spoken_languages: Option<Vec<Language>>,
    pub status: Option<String>,
    pub tagline: Option<String>,
    pub title: Option<String>,
    pub video: Option<bool>,
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
pub struct ProductionCountry {
    pub name: String,
    pub iso_3166_1: String,
}

#[derive(Deserialize, Serialize)]
pub struct ReleaseDate {
    #[serde(rename = "type")]
    pub release_type: i32,
    pub release_date: String,
    pub iso_639_1: Option<String>,
    pub descriptors: Option<Vec<String>>,
    pub note: Option<String>,
    pub certification: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ReleaseDates {
    pub iso_3166_1: String,
    pub release_dates: Vec<ReleaseDate>,
}

#[derive(Deserialize, Serialize)]
pub struct Cast {
    pub adult: Option<bool>,
    pub gender: Option<i32>,
    pub id: u64,
    pub known_for_department: Option<String>,
    pub name: Option<String>,
    pub original_name: Option<String>,
    pub popularity: Option<f32>,
    pub profile_path: Option<String>,
    pub cast_id: Option<u64>,
    pub character: Option<String>,
    pub credit_id: Option<String>,
    pub order: i32,
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

#[derive(Deserialize, Serialize)]
pub struct Video {
    #[serde(rename = "iso_639_1")]
    pub language: Option<String>,
    #[serde(rename = "iso_3166_1")]
    pub country: Option<String>,
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
pub struct MovieReleaseDatesResponse {
    pub id: u64,
    pub results: Vec<ReleaseDates>,
}

#[derive(Deserialize, Serialize)]
pub struct MovieCreditsResponse {
    pub id: u64,
    pub cast: Vec<Cast>,
    pub crew: Vec<Crew>,
}

#[derive(Deserialize, Serialize)]
pub struct MovieVideosResponse {
    pub id: u64,
    pub results: Vec<Video>,
}

#[derive(Deserialize, Serialize)]
pub struct MovieRecommendationsResponse {
    pub total_results: u64,
    pub total_pages: i32,
    pub page: i32,
    pub results: Vec<MovieOverview>,
}

#[derive(Deserialize, Serialize)]
pub struct SimilarMoviesResponse {
    pub total_results: u64,
    pub total_pages: i32,
    pub page: i32,
    pub results: Vec<MovieOverview>,
}
