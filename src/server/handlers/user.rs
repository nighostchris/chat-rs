use crate::db;
use crate::external::db::user::NewUser;
use crate::server::handlers::{ErrorResponse, HealthCheck, SuccessResponse};
use crate::server::ServerState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use bcrypt::{hash, DEFAULT_COST};
use dotenvy::var;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::ops::Add;
use std::sync::Arc;
use time::{Duration, OffsetDateTime};
use tracing::{error, info};
use uuid::Uuid;

use super::CustomJson;

#[derive(Clone, Debug, Deserialize)]
pub struct RegisterSchema {
    email: String,
    password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: Uuid,
    iss: String,
    exp: u64,
}

// #[derive(Debug, Serialize)]
// pub struct RegisterResponse {
//     success: bool,
//     result: {
//         message: String,
//     }
// }

// Handler function for path '/api/v1/user/register'
#[tracing::instrument]
pub async fn register_handler(
    State(state): State<Arc<ServerState>>,
    CustomJson(body): CustomJson<RegisterSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    info!("received request");
    let is_user_exists = db::user::is_user_exists(&state.db, body.email.as_str()).await?;

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
    let hashed_password = hash(body.password, DEFAULT_COST).map_err(|error| {
        error!("password hashing error. {}", error);
        // Will not continue if there is error during password hashing process
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                success: false,
                error: format!("Internal server error. Please try to register again."),
            }),
        );
    })?;

    // Insert a new user record into database
    let user_id = db::user::insert_new_user(
        &state.db,
        NewUser {
            email: body.email.clone(),
            // We can safely unwrap this as we will already end the process in section above if we encounter hashing error
            password: hashed_password,
        },
    )
    .await?;

    // Gather JWT access token related environment variable values
    let token_iss = var("TOKEN_ISS").map_err(|_| {
        error!("missing environment variable TOKEN_ISS");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                success: false,
                error: format!("Internal server error. Please try to register again."),
            }),
        );
    })?;

    let access_token_secret = var("ACCESS_TOKEN_SECRET").map_err(|_| {
        error!("missing environment variable ACCESS_TOKEN_SECRET");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                success: false,
                error: format!("Internal server error. Please try to register again."),
            }),
        );
    })?;

    // Construct JWT access token
    let access_token = encode(
        &Header::default(),
        &Claims {
            sub: user_id,
            iss: token_iss,
            exp: OffsetDateTime::now_utc()
                .add(Duration::minutes(5))
                .unix_timestamp()
                .unsigned_abs(),
        },
        &EncodingKey::from_secret(access_token_secret.as_ref()),
    )
    .map_err(|error| {
        error!("jwt access token construction error. {}", error);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                success: false,
                error: format!("Internal server error. Please try to register again."),
            }),
        );
    })?;

    // Construct cookie for the JWT access token
    // let cookie = Cookie::build()

    Ok((
        StatusCode::OK,
        Json(SuccessResponse::<HealthCheck> {
            success: true,
            result: HealthCheck {},
        }),
    ))
}
