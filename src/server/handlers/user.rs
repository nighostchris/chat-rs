use crate::db;
use crate::external::db::user::NewUser;
use crate::server::handlers::{ErrorResponse, HealthCheckResponse};
use crate::server::ServerState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use bcrypt::{hash, DEFAULT_COST};
use serde::Deserialize;
use std::sync::Arc;
use tracing::{error, info};

use super::CustomJson;

#[derive(Debug, Deserialize)]
pub struct RegisterSchema {
    email: String,
    password: String,
}

// Handler function for path '/api/v1/user/register'
#[tracing::instrument]
pub async fn register_handler(
    State(state): State<Arc<ServerState>>,
    CustomJson(body): CustomJson<RegisterSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    info!("received request");
    let is_user_exists = db::user::is_user_exists(&state.db, body.email).await?;

    // Will not continue the registration if email already exists in database
    if is_user_exists {
        return Err((
            StatusCode::CONFLICT,
            Json(ErrorResponse {
                success: false,
                error: format!("User already exists."),
            }),
        ));
    }

    // Generate hashed password for user
    let hashed_password = hash(body.password, DEFAULT_COST);
    // Will not continue if there is error during password hashing process
    if let Err(hash_error) = hashed_password {
        error!("password hashing error. {}", hash_error);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                success: false,
                error: format!("Internal server error. Please try to register again."),
            }),
        ));
    }

    let inserted_user = db::user::insert_new_user(
        &state.db,
        NewUser {
            email: format!("test@gmail.com"),
            // We can safely unwrap this as we will already end the process in section above if we encounter hashing error
            password: hashed_password.unwrap(),
        },
    )
    .await?;

    Ok((StatusCode::OK, Json(HealthCheckResponse { success: true })))
}
