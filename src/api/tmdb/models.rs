use axum::{
    Json,
    response::{IntoResponse, Response},
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Genre {
    id: u64,
    name: String,
}

#[derive(Deserialize, Serialize)]
pub struct Language {
    iso_639_1: String,
    english_name: String,
    name: String,
}

impl IntoResponse for Language {
    fn into_response(self) -> Response {
        let body = Json(self);
        (StatusCode::OK, body).into_response()
    }
}

#[derive(Deserialize, Serialize)]
pub struct ProductionCompany {
    id: u64,
    name: Option<String>,
    origin_country: Option<String>,
    logo_path: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ProductionCountry {
    name: String,
    iso_3166_1: String,
}

#[derive(Deserialize, Serialize)]
pub struct Collection {
    id: u64,
    name: String,
    poster_path: Option<String>,
    backdrop_path: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct MovieOverview {
    adult: Option<bool>,
    backdrop_path: Option<String>,
    poster_path: Option<String>,
    genre_ids: Option<Vec<u64>>,
    pub id: u64,
    original_language: Option<String>,
    original_title: Option<String>,
    title: Option<String>,
    overview: Option<String>,
    popularity: Option<f32>,
    release_date: Option<String>,
    video: Option<bool>,
    vote_average: Option<f32>,
    vote_count: Option<u64>,
}

#[derive(Deserialize, Serialize)]
pub struct MovieDetails {
    id: u64,
    imdb_id: Option<String>,
    adult: Option<bool>,
    backdrop_path: Option<String>,
    poster_path: Option<String>,
    belongs_to_collection: Option<Collection>,
    budget: Option<i64>,
    genres: Option<Vec<Genre>>,
    homepage: Option<String>,
    origin_country: Option<Vec<String>>,
    original_language: Option<String>,
    original_title: Option<String>,
    overview: Option<String>,
    popularity: Option<f32>,
    production_companies: Option<Vec<ProductionCompany>>,
    production_countries: Option<Vec<ProductionCountry>>,
    release_date: Option<String>,
    revenue: Option<i64>,
    runtime: Option<i64>,
    spoken_languages: Option<Vec<Language>>,
    status: Option<String>,
    tagline: Option<String>,
    title: Option<String>,
    video: Option<bool>,
    vote_average: Option<f32>,
    vote_count: Option<i64>,
}

impl IntoResponse for MovieDetails {
    fn into_response(self) -> Response {
        let body = Json(self);
        (StatusCode::OK, body).into_response()
    }
}

#[derive(Deserialize, Serialize)]
pub struct ReleaseDate {
    iso_639_1: String,
    #[serde(rename = "type")]
    release_type: i32,
    release_date: String,
    descriptors: Option<Vec<String>>,
    note: Option<String>,
    certification: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ReleaseDates {
    iso_3166_1: String,
    release_dates: Vec<ReleaseDate>,
}
