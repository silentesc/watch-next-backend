use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum SortBy {
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
    #[serde(rename = "page")]
    page: Option<i32>,
    #[serde(rename = "primary_release_date.gte")]
    primary_release_date_gte: Option<String>,
    #[serde(rename = "primary_release_date.lte")]
    primary_release_date_lte: Option<String>,
    #[serde(rename = "sort_by")]
    sort_by: Option<SortBy>,
    #[serde(rename = "vote_average.gte")]
    vote_average_gte: Option<f32>,
    #[serde(rename = "vote_average.lte")]
    vote_average_lte: Option<f32>,
    #[serde(rename = "vote_count.gte")]
    vote_count_gte: Option<f32>,
    #[serde(rename = "vote_count.lte")]
    vote_count_lte: Option<f32>,
    #[serde(rename = "with_genres")]
    with_genres: Option<String>,
    #[serde(rename = "without_genres")]
    without_genres: Option<String>,
    #[serde(rename = "with_origin_country")]
    with_origin_country: Option<String>,
    #[serde(rename = "with_original_language")]
    with_original_language: Option<String>,
    #[serde(rename = "with_runtime.gte")]
    with_runtime_gte: Option<i32>,
    #[serde(rename = "with_runtime.lte")]
    with_runtime_lte: Option<i32>,
}
