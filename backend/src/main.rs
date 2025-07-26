/*
File: src/main.rs
Purpose: Main entry point for the Axum web application template.
*/

// Declare the modules that make up the application.
mod config;
mod db;
mod errors;
mod handlers;
mod models;
mod routes;

use crate::config::AppConfig;
use anyhow::{Context, Result};
use axum::http::{HeaderValue, Method};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

// The shared application state, intended to hold the database pool and other shared resources.
pub struct AppState {
    pub config: AppConfig,
    // Example: pub db_pool: sqlx::PgPool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // --- 1. Initialize Logging ---
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axum_template=info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // --- 2. Load Application Configuration ---
    let app_config = AppConfig::load().context("Failed to load application configuration")?;

    // --- 3. Initialize Database Connection ---
    // let db_pool = db::create_pool(&app_config.database_url).await?;
    // tracing::info!("Database connection pool created.");

    // --- 4. Initialize Shared Application State ---
    let app_state = Arc::new(AppState {
        config: app_config.clone(),
        // db_pool,
    });

    // --- 5. Configure CORS (Cross-Origin Resource Sharing) ---
    let cors = CorsLayer::new()
        .allow_origin(app_config.frontend_origin.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([
            "content-type".parse().unwrap(),
            "authorization".parse().unwrap(),
        ]);

    // --- 6. Define Application Routes ---
    let app = routes::create_router(app_state.clone()).layer(cors);

    // --- 7. Start the HTTP Server ---
    let addr = SocketAddr::from(([0, 0, 0, 0], app_config.app_port));
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("🦀 Server listening on http://{} 🦀", addr);
    tracing::info!(
        "Backend is ready to serve API requests from the configured frontend origin: {}.",
        app_state.config.frontend_origin
    );

    axum::serve(listener, app).await?;

    Ok(())
}
