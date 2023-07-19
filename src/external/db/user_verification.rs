use crate::server::handlers::ErrorResponse;
use axum::{http::StatusCode, Json};
use sqlx::{Pool, Postgres};
use tracing::error;
use uuid::Uuid;

#[derive(Debug)]
pub struct NewUserVerification {
    pub user_id: Uuid,
    pub secret: String,
}

#[tracing::instrument]
pub async fn get_user_verification_secret(
    db_client: &Pool<Postgres>,
    user_id: &Uuid,
) -> Result<String, (StatusCode, Json<ErrorResponse>)> {
    let user_verification_secret = sqlx::query_scalar!(
        "SELECT secret FROM user_verification WHERE user_id = $1",
        user_id
    )
    .fetch_one(db_client)
    .await
    .map_err(|error| {
        error!(
            "failed to get user verification secret from database. {}",
            error
        );
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                success: false,
                error: format!("Internal server error."),
            }),
        );
    })?;

    return Ok(user_verification_secret);
}

#[tracing::instrument]
pub async fn insert_new_user_verification(
    db_client: &Pool<Postgres>,
    user_verification: NewUserVerification,
) -> Result<(), (StatusCode, Json<ErrorResponse>)> {
    sqlx::query!(
        "INSERT INTO user_verification (user_id, secret) VALUES ($1, $2)",
        user_verification.user_id,
        user_verification.secret
    )
    .execute(db_client)
    .await
    .map_err(|error| {
        error!(
            "failed to insert new user verification record into database. {}",
            error
        );
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                success: false,
                error: format!("Internal server error."),
            }),
        );
    })?;

    Ok(())
}
