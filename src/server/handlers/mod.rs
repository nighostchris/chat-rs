pub mod user;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;
use tracing::info;

#[derive(Debug, Serialize)]
pub struct HealthCheckResponse {
    pub success: bool,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
}

// Handler function for path '/'
#[tracing::instrument]
pub async fn health_check_handler() -> impl IntoResponse {
    info!("received request");
    (StatusCode::OK, Json(HealthCheckResponse { success: true }))
}
