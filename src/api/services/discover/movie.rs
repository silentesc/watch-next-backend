use crate::api::{
    errors::AppError,
    models::discover::{requests::DiscoverMovieParams, responses::DiscoverMovieResponse},
    tmdb::client::TmdbClient,
};

pub async fn discover(client: TmdbClient, params: DiscoverMovieParams) -> Result<DiscoverMovieResponse, AppError> {
    let mut response: DiscoverMovieResponse = client.get("/discover/movie", &params).await?;

    let mut seen_ids = std::collections::HashSet::new();
    response.results.retain(|movie| seen_ids.insert(movie.id));
    Ok(response)
}
