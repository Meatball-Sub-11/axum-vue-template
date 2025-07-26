/*
File: src/errors.rs
Purpose: Defines custom error types for the application.
*/
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use validator::ValidationErrors;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Internal Server Error")]
    Internal(#[from] anyhow::Error),

    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Not Found: {0}")]
    NotFound(String),

    #[error("Validation Error: {0}")]
    Validation(#[from] ValidationErrors),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Internal(e) => {
                tracing::error!("Internal server error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An unexpected server error occurred.".to_string(),
                )
            }
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Validation(errors) => {
                // Create a simplified error message for validation failures.
                let mut messages = Vec::new();
                for (field, field_errors) in errors.field_errors() {
                    for error in field_errors {
                        messages.push(format!(
                            "{}: {}",
                            field,
                            error.message.as_deref().unwrap_or("invalid value")
                        ));
                    }
                }
                (StatusCode::BAD_REQUEST, messages.join(", "))
            }
        };

        let body = Json(json!({
            "success": false,
            "message": error_message,
        }));

        (status, body).into_response()
    }
}
