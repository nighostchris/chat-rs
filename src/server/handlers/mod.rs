pub mod user;

use axum::extract::rejection::{JsonRejection, QueryRejection};
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use axum_macros::{FromRequest, FromRequestParts};
use serde::Serialize;
use tracing::{error, info};

#[derive(FromRequest)]
#[from_request(via(Json), rejection(CustomError))]
pub struct CustomJson<T>(T);

#[derive(FromRequestParts)]
#[from_request(via(Query), rejection(CustomError))]
pub struct CustomQuery<T>(T);

pub struct CustomError {
    status: StatusCode,
    message: String,
}

#[derive(Debug, Serialize)]
pub struct HealthCheck;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse<T> {
    success: bool,
    result: T,
}

impl From<JsonRejection> for CustomError {
    fn from(rejection: JsonRejection) -> Self {
        Self {
            status: rejection.status(),
            message: rejection.body_text(),
        }
    }
}

impl From<QueryRejection> for CustomError {
    fn from(rejection: QueryRejection) -> Self {
        Self {
            status: rejection.status(),
            message: rejection.body_text(),
        }
    }
}

impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        error!("{}", self.message);
        // format!("Missing required fields in request body.")
        let error_message = match self.status {
            StatusCode::BAD_REQUEST => {
                if self.message.contains("missing field") {
                    let message_shards: Vec<&str> = self.message.split('`').collect();
                    let field_name = message_shards[1].trim();
                    format!("Missing required field {} in query.", field_name)
                } else {
                    format!("Invalid request.")
                }
            }
            StatusCode::UNSUPPORTED_MEDIA_TYPE => format!("Expected non-empty request body."),
            StatusCode::UNPROCESSABLE_ENTITY => {
                if self.message.contains("invalid type") {
                    let message_shards: Vec<&str> = self.message.split(':').collect();
                    let field_name = message_shards[1].trim();
                    let field_type_shards: Vec<&str> =
                        message_shards[3].trim().split(' ').collect();
                    let field_type = field_type_shards[0].trim();
                    format!("Invalid data type {} for field {}.", field_type, field_name)
                } else if self.message.contains("missing field") {
                    let message_shards: Vec<&str> = self.message.split('`').collect();
                    let field_name = message_shards[1].trim();
                    format!("Missing required field {} in request body.", field_name)
                } else {
                    format!("Unable to process request body.")
                }
            }
            _ => self.message,
        };
        (
            self.status,
            Json(ErrorResponse {
                success: false,
                error: error_message,
            }),
        )
            .into_response()
    }
}

// Handler function for path '/'
#[tracing::instrument]
pub async fn health_check_handler() -> impl IntoResponse {
    info!("received request");
    (
        StatusCode::OK,
        Json(SuccessResponse::<HealthCheck> {
            success: true,
            result: HealthCheck {},
        }),
    )
}
