/*
File: src/routes.rs
Purpose: Defines all API routes for the application.
*/
use crate::{AppState, handlers};
use axum::{Router, routing::get};
use std::sync::Arc;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        // A simple health check endpoint to confirm the server is running.
        .route("/api/health", get(handlers::health_check))
        // Application-specific routes are added here.
        .with_state(app_state)
}
