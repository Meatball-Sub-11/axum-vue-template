/*
File: src/models.rs
Purpose: Defines common data structures used for API requests and responses.
*/
use serde::Serialize;

/// A generic response structure for simple API messages.
#[derive(Serialize)]
pub struct ApiResponse {
    pub status: String,
    pub message: String,
}

// Application-specific request and response structs (e.g., CreateUserRequest, ItemResponse)
// should be added below.
