use crate::{
    app::errors::AppError,
    features::collections::dto::{CollectionDetails, CollectionDetailsParams},
    integrations::tmdb::client::TmdbClient,
};

pub async fn get_collection_details(
    client: TmdbClient,
    collection_id: i32,
    params: CollectionDetailsParams,
) -> Result<CollectionDetails, AppError> {
    client
        .get(format!("/collection/{collection_id}").as_str(), &params)
        .await
        .map_err(Into::into)
}
