use crate::server::handlers::ErrorResponse;
use axum::{http::StatusCode, Json};
use sqlx::{Pool, Postgres};
use tracing::error;
use uuid::Uuid;

#[derive(Debug)]
pub struct NewUser {
    pub email: String,
    pub password: String,
}

#[tracing::instrument]
pub async fn is_user_exists(
    db_client: &Pool<Postgres>,
    email: &str,
) -> Result<bool, (StatusCode, Json<ErrorResponse>)> {
    let user_exists_result = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM \"user\" WHERE email = $1)",
        email
    )
    .fetch_one(db_client)
    .await;

    match user_exists_result {
        Ok(user_exists) => match user_exists {
            Some(result) => Ok(result),
            None => Ok(false),
        },
        Err(e) => {
            error!("{}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    success: false,
                    error: format!("Internal server error."),
                }),
            ))
        }
    }
}

#[tracing::instrument]
pub async fn insert_new_user(
    db_client: &Pool<Postgres>,
    new_user: NewUser,
) -> Result<Uuid, (StatusCode, Json<ErrorResponse>)> {
    let inserted_user_id = sqlx::query_scalar!(
        "INSERT INTO \"user\" (email, password) VALUES ($1, $2) RETURNING id",
        new_user.email,
        new_user.password,
    )
    .fetch_one(db_client)
    .await
    .map_err(|error| {
        error!("{}", error);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                success: false,
                error: format!("Internal server error."),
            }),
        )
    });
    inserted_user_id
}

#[tracing::instrument]
pub async fn get_user_password(
    db_client: &Pool<Postgres>,
    email: &str,
) -> Result<String, (StatusCode, Json<ErrorResponse>)> {
    let user_password =
        sqlx::query_scalar!("SELECT password FROM \"user\" WHERE email = $1", email)
            .fetch_one(db_client)
            .await
            .map_err(|error| {
                error!("{}", error);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        success: false,
                        error: format!("Internal server error."),
                    }),
                )
            });
    user_password
}

#[tracing::instrument]
pub async fn update_verified_status(
    db_client: &Pool<Postgres>,
    user_id: &Uuid,
    status: bool,
) -> Result<(), (StatusCode, Json<ErrorResponse>)> {
    sqlx::query!(
        "UPDATE \"user\" SET verified = $1 WHERE id = $2",
        status,
        user_id
    )
    .execute(db_client)
    .await
    .map_err(|error| {
        error!("{}", error);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                success: false,
                error: format!("Internal server error."),
            }),
        )
    })?;
    Ok(())
}
