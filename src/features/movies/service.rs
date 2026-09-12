use std::collections::HashSet;

use crate::{
    app::errors::AppError,
    features::movies::{dto::*, repository},
    integrations::tmdb::client::TmdbClient,
};

pub async fn get_details(
    client: &TmdbClient,
    movie_id: i32,
    params: &MovieDetailsParams,
) -> Result<MovieDetails, AppError> {
    repository::get_details(client, movie_id, params).await
}

pub async fn get_credits(
    client: &TmdbClient,
    movie_id: i32,
    params: &MovieCreditsParams,
) -> Result<MovieCreditsResponse, AppError> {
    repository::get_credits(client, movie_id, params).await
}

pub async fn get_recommendations(
    client: &TmdbClient,
    movie_id: i32,
    params: &MovieRecommendationsParams,
) -> Result<MovieRecommendationsResponse, AppError> {
    let mut response = repository::get_recommendations(client, movie_id, params).await?;
    let mut seen_ids = HashSet::new();
    response.results.retain(|movie| seen_ids.insert(movie.id));
    Ok(response)
}

pub async fn get_release_dates(client: &TmdbClient, movie_id: i32) -> Result<MovieReleaseDatesResponse, AppError> {
    repository::get_release_dates(client, movie_id).await
}

pub async fn get_similar(
    client: &TmdbClient,
    movie_id: i32,
    params: &SimilarMoviesParams,
) -> Result<SimilarMoviesResponse, AppError> {
    let mut response = repository::get_similar(client, movie_id, params).await?;
    let mut seen_ids = HashSet::new();
    response.results.retain(|movie| seen_ids.insert(movie.id));
    Ok(response)
}

pub async fn get_videos(
    client: &TmdbClient,
    movie_id: i32,
    params: &MovieVideosParams,
) -> Result<MovieVideosResponse, AppError> {
    repository::get_videos(client, movie_id, params).await
}
