use chrono::NaiveDateTime;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    api::{db::models::Session, errors::AppError},
    error,
    logger::enums::category::Category,
};

/**
 * Get session by session id
 */
pub async fn get_session_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Session>, AppError> {
    let session: Option<Session> = match sqlx::query_as("SELECT * FROM sessions WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await
    {
        Ok(session) => session,
        Err(err) => {
            error!(Category::Db, "Getting session by id failed with error: {:#?}", err);
            return Err(AppError::generic_500());
        }
    };

    Ok(session)
}

/**
 * Create session and get session id
 */
pub async fn create_session(pool: &PgPool, user_id: Uuid, expires_at: NaiveDateTime) -> Result<Uuid, AppError> {
    let session_id: (Uuid,) =
        match sqlx::query_as("INSERT INTO sessions (user_id, expires_at) VALUES ($1, $2) RETURNING id")
            .bind(user_id)
            .bind(expires_at)
            .fetch_one(pool)
            .await
        {
            Ok(session_id) => session_id,
            Err(err) => {
                error!(Category::Db, "Creating session failed with error: {:#?}", err);
                return Err(AppError::generic_500());
            }
        };

    let session_id = session_id.0;

    Ok(session_id)
}

/**
 * Delete a session
 */
pub async fn delete_session(pool: &PgPool, session_id: Uuid) -> Result<(), AppError> {
    match sqlx::query("DELETE FROM sessions WHERE id = $1")
        .bind(session_id)
        .execute(pool)
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => {
            error!(Category::Db, "Deleting session failed with error: {:#?}", err);
            Err(AppError::generic_500())
        }
    }
}
