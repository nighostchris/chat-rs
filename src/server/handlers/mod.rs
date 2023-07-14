pub mod user;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;
use tracing::info;

#[derive(Serialize)]
pub struct HealthCheckResponse {
    success: bool,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    success: bool,
    error: String,
}

// Handler function for path '/'
#[tracing::instrument]
pub async fn health_check_handler() -> impl IntoResponse {
    info!("received request");
    (StatusCode::OK, Json(HealthCheckResponse { success: true }))
}
