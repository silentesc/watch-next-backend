use crate::{
    app::errors::AppError,
    integrations::tmdb::{
        TmdbApi,
        models::movies::MovieDetails,
        resources::movies::dto::{
            MovieCreditsParams, MovieCreditsResponse, MovieDetailsParams, MovieRecommendationsParams,
            MovieRecommendationsResponse, MovieReleaseDatesResponse, MovieVideosParams, MovieVideosResponse,
            SimilarMoviesParams, SimilarMoviesResponse,
        },
    },
};

pub async fn get_details(tmdb: TmdbApi, movie_id: i32, params: MovieDetailsParams) -> Result<MovieDetails, AppError> {
    tmdb.movies().details(movie_id, params).await.map_err(Into::into)
}

pub async fn get_credits(
    tmdb: TmdbApi,
    movie_id: i32,
    params: MovieCreditsParams,
) -> Result<MovieCreditsResponse, AppError> {
    tmdb.movies().credits(movie_id, params).await.map_err(Into::into)
}

pub async fn get_recommendations(
    tmdb: TmdbApi,
    movie_id: i32,
    params: MovieRecommendationsParams,
) -> Result<MovieRecommendationsResponse, AppError> {
    tmdb.movies()
        .recommendations(movie_id, params)
        .await
        .map_err(Into::into)
}

pub async fn get_release_dates(tmdb: TmdbApi, movie_id: i32) -> Result<MovieReleaseDatesResponse, AppError> {
    tmdb.movies().release_dates(movie_id).await.map_err(Into::into)
}

pub async fn get_similar(
    tmdb: TmdbApi,
    movie_id: i32,
    params: SimilarMoviesParams,
) -> Result<SimilarMoviesResponse, AppError> {
    tmdb.movies().similar(movie_id, params).await.map_err(Into::into)
}

pub async fn get_videos(
    tmdb: TmdbApi,
    movie_id: i32,
    params: MovieVideosParams,
) -> Result<MovieVideosResponse, AppError> {
    tmdb.movies().videos(movie_id, params).await.map_err(Into::into)
}
