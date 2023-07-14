use crate::db;
use crate::server::handlers::{ErrorResponse, HealthCheckResponse};
use crate::server::ServerState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use std::sync::Arc;
use tracing::{error, info};

// Handler function for path '/api/v1/user/register'
#[tracing::instrument]
pub async fn register_handler(
    State(state): State<Arc<ServerState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    info!("received request");
    // TO FIX: Remove hard coded test and get request body
    let is_user_exists = db::user::is_user_exists(&state.db, "test".to_string()).await;

    match is_user_exists {
        // Will not continue the registration if email already exists in database
        Ok(user_exists) => {
            if user_exists {
                return Err((
                    StatusCode::CONFLICT,
                    Json(ErrorResponse {
                        success: false,
                        error: format!("User already exists."),
                    }),
                ));
            }
        }
        // Will not continue if database failed to check user email existence
        Err(error) => {
            error!("{}", format!("{}", error));
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    success: false,
                    error: format!("Internal server error. Please try to register again."),
                }),
            ));
        }
    }

    Ok((StatusCode::OK, Json(HealthCheckResponse { success: true })))
}
