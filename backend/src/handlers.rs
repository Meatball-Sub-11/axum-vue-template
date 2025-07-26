/*
File: src/handlers.rs
Purpose: Contains the HTTP handler functions for the Axum routes.
*/
use crate::models::ApiResponse;
use axum::Json;

/// Handles GET requests to the `/api/health` endpoint.
/// Returns a simple JSON response indicating the API is operational.
pub async fn health_check() -> Json<ApiResponse> {
    Json(ApiResponse {
        status: "success".to_string(),
        message: "API is up and running!".to_string(),
    })
}
