use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum MovieSortBy {
    #[serde(rename = "original_title.asc")]
    OriginalTitleAsc,
    #[serde(rename = "original_title.desc")]
    OriginalTitleDesc,
    #[serde(rename = "popularity.asc")]
    PopularityAsc,
    #[serde(rename = "popularity.desc")]
    PopularityDesc,
    #[serde(rename = "revenue.asc")]
    RevenueAsc,
    #[serde(rename = "revenue.desc")]
    RevenueDesc,
    #[serde(rename = "primary_release_date.asc")]
    PrimaryReleaseDateAsc,
    #[serde(rename = "primary_release_date.desc")]
    PrimaryReleaseDateDesc,
    #[serde(rename = "title.asc")]
    TitleAsc,
    #[serde(rename = "title.desc")]
    TitleDesc,
    #[serde(rename = "vote_average.asc")]
    VoteAverageAsc,
    #[serde(rename = "vote_average.desc")]
    VoteAverageDesc,
    #[serde(rename = "vote_count.asc")]
    VoteCountAsc,
    #[serde(rename = "vote_count.desc")]
    VoteCountDesc,
}

#[derive(Deserialize, Serialize)]
pub struct DiscoverMovieParams {
    pub page: Option<i32>,
    #[serde(rename = "primary_release_date.gte")]
    pub primary_release_date_gte: Option<String>,
    #[serde(rename = "primary_release_date.lte")]
    pub primary_release_date_lte: Option<String>,
    pub sort_by: Option<MovieSortBy>,
    #[serde(rename = "vote_average.gte")]
    pub vote_average_gte: Option<f32>,
    #[serde(rename = "vote_average.lte")]
    pub vote_average_lte: Option<f32>,
    #[serde(rename = "vote_count.gte")]
    pub vote_count_gte: Option<f32>,
    #[serde(rename = "vote_count.lte")]
    pub vote_count_lte: Option<f32>,
    pub with_genres: Option<String>,
    pub without_genres: Option<String>,
    pub with_origin_country: Option<String>,
    pub with_original_language: Option<String>,
    #[serde(rename = "with_runtime.gte")]
    pub with_runtime_gte: Option<i32>,
    #[serde(rename = "with_runtime.lte")]
    pub with_runtime_lte: Option<i32>,
}

#[derive(Deserialize, Serialize)]
pub enum TvSortBy {
    #[serde(rename = "original_name.asc")]
    OriginalNameAsc,
    #[serde(rename = "original_name.desc")]
    OriginalNameDesc,
    #[serde(rename = "popularity.asc")]
    PopularityAsc,
    #[serde(rename = "popularity.desc")]
    PopularityDesc,
    #[serde(rename = "first_air_date.asc")]
    FirstAirDateAsc,
    #[serde(rename = "first_air_date.desc")]
    FirstAirDateDesc,
    #[serde(rename = "name.asc")]
    NameAsc,
    #[serde(rename = "name.desc")]
    NameDesc,
    #[serde(rename = "vote_average.asc")]
    VoteAverageAsc,
    #[serde(rename = "vote_average.desc")]
    VoteAverageDesc,
    #[serde(rename = "vote_count.asc")]
    VoteCountAsc,
    #[serde(rename = "vote_count.desc")]
    VoteCountDesc,
}

#[derive(Deserialize, Serialize)]
pub struct DiscoverTvParams {
    pub page: Option<i32>,
    #[serde(rename = "first_air_date.gte")]
    pub first_air_date_gte: Option<String>,
    #[serde(rename = "first_air_date.lte")]
    pub first_air_date_lte: Option<String>,
    pub sort_by: Option<TvSortBy>,
    #[serde(rename = "vote_average.gte")]
    pub vote_average_gte: Option<f32>,
    #[serde(rename = "vote_average.lte")]
    pub vote_average_lte: Option<f32>,
    #[serde(rename = "vote_count.gte")]
    pub vote_count_gte: Option<f32>,
    #[serde(rename = "vote_count.lte")]
    pub vote_count_lte: Option<f32>,
    pub with_genres: Option<String>,
    pub without_genres: Option<String>,
    pub with_origin_country: Option<String>,
    pub with_original_language: Option<String>,
    #[serde(rename = "with_runtime.gte")]
    pub with_runtime_gte: Option<i32>,
    #[serde(rename = "with_runtime.lte")]
    pub with_runtime_lte: Option<i32>,
    pub with_status: Option<String>,
}
