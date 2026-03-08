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
    id: i32,
    name: String,
    origin_country: String,
    logo_path: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ProductionCountry {
    name: String,
    iso_3166_1: String,
}

#[derive(Deserialize, Serialize)]
pub struct Collection {
    id: i32,
    name: String,
    poster_path: Option<String>,
    backdrop_path: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct MovieOverview {
    adult: bool,
    backdrop_path: Option<String>,
    poster_path: Option<String>,
    genre_ids: Vec<u64>,
    id: u64,
    original_language: String,
    original_title: String,
    title: String,
    overview: String,
    popularity: f32,
    release_date: String,
    video: bool,
    vote_average: f32,
    vote_count: u64,
}

#[derive(Deserialize, Serialize)]
pub struct MovieDetails {
    id: i32,
    imdb_id: Option<String>,
    adult: bool,
    backdrop_path: Option<String>,
    belongs_to_collection: Option<Collection>,
    budget: i32,
    genres: Vec<Genre>,
    homepage: Option<String>,
    origin_country: Option<Vec<String>>,
    original_language: String,
    original_title: String,
    overview: String,
    popularity: f32,
    production_companies: Vec<ProductionCompany>,
    production_countries: Vec<ProductionCountry>,
    release_date: String,
    revenue: i32,
    runtime: i32,
    spoken_languages: Vec<Language>,
    status: String,
    tagline: String,
    title: String,
    video: bool,
    vote_average: f32,
    vote_count: i32,
}

impl IntoResponse for MovieDetails {
    fn into_response(self) -> Response {
        let body = Json(self);
        (StatusCode::OK, body).into_response()
    }
}
