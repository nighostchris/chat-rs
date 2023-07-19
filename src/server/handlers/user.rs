use crate::db;
use crate::external::db::user::NewUser;
use crate::external::db::user_verification::NewUserVerification;
use crate::server::handlers::{ErrorResponse, SuccessResponse};
use crate::server::ServerState;
use axum::extract::State;
use axum::http::{header, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use axum_extra::extract::cookie::{Cookie, SameSite};
use bcrypt::{hash, DEFAULT_COST};
use dotenvy::var;
use jsonwebtoken::{encode, EncodingKey, Header};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use std::ops::Add;
use std::sync::Arc;
use time::{Duration, OffsetDateTime};
use tracing::{debug, error, info};
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

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    message: String,
    verification_token: String,
}

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

    debug!("going to generate hashed password");
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

    debug!("going to insert new user record into database");
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

    debug!("constructing jwt access token");
    // Construct JWT access token
    let access_token = encode(
        &Header::default(),
        &Claims {
            sub: user_id,
            iss: token_iss.clone(),
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

    // Generate a random 64 bytes long secret to sign the verification token for the user
    let mut random_bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut random_bytes);
    let verification_secret = random_bytes
        .iter()
        .map(|byte| format!("{:x}", byte).to_string())
        .collect::<Vec<String>>()
        .join("");

    debug!("inserting new user verification record into database");
    // Insert a new user verification record
    db::user_verification::insert_new_user_verification(
        &state.db,
        NewUserVerification {
            user_id: user_id.clone(),
            secret: verification_secret.clone(),
        },
    )
    .await?;

    debug!("constructing jwt verification token");
    // Construct the verification token
    let verification_token = encode(
        &Header::default(),
        &Claims {
            sub: user_id,
            iss: token_iss,
            exp: OffsetDateTime::now_utc()
                .add(Duration::minutes(5))
                .unix_timestamp()
                .unsigned_abs(),
        },
        &EncodingKey::from_secret(verification_secret.as_ref()),
    )
    .map_err(|error| {
        error!("jwt verification token construction error. {}", error);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                success: false,
                error: format!("Internal server error. Please try to register again."),
            }),
        );
    })?;

    debug!("constructing cookie for JWT access token");
    // Construct cookie for the JWT access token
    let cookie = Cookie::build("token", access_token.to_owned())
        .path("/")
        .secure(false) // Forbid cookie from transmitting over simple HTTP
        .http_only(true) // Blocks access of related cookie from client side
        .same_site(SameSite::Lax) // SameSite 'none' has to be used together with secure - true
        .max_age(Duration::minutes(5))
        .finish(); // The duration better to align with expiry time of access token

    let mut response = (
        StatusCode::OK,
        Json(SuccessResponse::<RegisterResponse> {
            success: true,
            result: RegisterResponse {
                message: "User registration complete.".to_string(),
                // We should send email with the link of postfix containing the verification token to the user email
                // We are embedding the verification token here for easier development purpose
                verification_token,
            },
        }),
    )
        .into_response();

    // Embed the cookie in response
    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());

    Ok(response)
}
