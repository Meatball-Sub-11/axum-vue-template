/*
File: src/config.rs
Purpose: Handles loading and managing application configuration from environment variables.
*/
use anyhow::{Context, Result};
use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub jwt_secret: String,
    pub database_url: String,
    pub app_port: u16,
    pub frontend_origin: String,
}

impl AppConfig {
    /// Loads configuration by reading from environment variables.
    /// In development, it will first load variables from a `.env` file if present.
    pub fn load() -> Result<Self> {
        dotenv().ok();

        let config = envy::from_env::<AppConfig>()
            .context("Failed to load configuration from environment variables")?;

        Ok(config)
    }
}
