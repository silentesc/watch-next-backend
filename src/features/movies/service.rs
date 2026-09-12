use crate::{
    app::errors::AppError,
    features::movies::dto::{
        MovieCreditsParams, MovieCreditsResponse, MovieDetails, MovieDetailsParams, MovieRecommendationsParams,
        MovieRecommendationsResponse, MovieReleaseDatesResponse, MovieVideosParams, MovieVideosResponse,
        SimilarMoviesParams, SimilarMoviesResponse,
    },
    integrations::tmdb::client::TmdbClient,
};

pub async fn get_details(
    client: &TmdbClient,
    movie_id: i32,
    params: &MovieDetailsParams,
) -> Result<MovieDetails, AppError> {
    client.get(format!("/movie/{movie_id}").as_str(), params).await
}

pub async fn get_credits(
    client: &TmdbClient,
    movie_id: i32,
    params: &MovieCreditsParams,
) -> Result<MovieCreditsResponse, AppError> {
    client.get(format!("/movie/{movie_id}/credits").as_str(), params).await
}

pub async fn get_recommendations(
    client: &TmdbClient,
    movie_id: i32,
    params: &MovieRecommendationsParams,
) -> Result<MovieRecommendationsResponse, AppError> {
    client
        .get(format!("/movie/{movie_id}/recommendations").as_str(), params)
        .await
}

pub async fn get_release_dates(client: &TmdbClient, movie_id: i32) -> Result<MovieReleaseDatesResponse, AppError> {
    client
        .get(format!("/movie/{movie_id}/release_dates").as_str(), &())
        .await
}

pub async fn get_similar(
    client: &TmdbClient,
    movie_id: i32,
    params: &SimilarMoviesParams,
) -> Result<SimilarMoviesResponse, AppError> {
    client.get(format!("/movie/{movie_id}/similar").as_str(), params).await
}

pub async fn get_videos(
    client: &TmdbClient,
    movie_id: i32,
    params: &MovieVideosParams,
) -> Result<MovieVideosResponse, AppError> {
    client.get(format!("/movie/{movie_id}/videos").as_str(), params).await
}
