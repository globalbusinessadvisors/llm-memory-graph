//! LLM-Registry integration client
//!
//! Provides a client for interacting with the LLM-Registry service,
//! which tracks model metadata, versions, and capabilities.

use crate::{Error, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::time::Duration;

/// Configuration for LLM-Registry client
#[derive(Debug, Clone)]
pub struct RegistryConfig {
    /// Base URL of the registry service
    pub base_url: String,
    /// API key for authentication
    pub api_key: String,
    /// Request timeout in seconds
    pub timeout_secs: u64,
    /// Maximum retry attempts
    pub max_retries: usize,
}

impl RegistryConfig {
    /// Create a new registry configuration
    pub fn new(base_url: impl Into<String>, api_key: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            api_key: api_key.into(),
            timeout_secs: 30,
            max_retries: 3,
        }
    }

    /// Set request timeout
    pub fn with_timeout(mut self, timeout_secs: u64) -> Self {
        self.timeout_secs = timeout_secs;
        self
    }

    /// Set maximum retries
    pub fn with_max_retries(mut self, max_retries: usize) -> Self {
        self.max_retries = max_retries;
        self
    }
}

/// Model metadata stored in the registry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetadata {
    /// Model name
    pub name: String,
    /// Model version
    pub version: String,
    /// Model provider (e.g., "OpenAI", "Anthropic")
    pub provider: String,
    /// Context window size
    pub context_window: usize,
    /// Model capabilities (JSON metadata)
    pub capabilities: JsonValue,
    /// When the model was registered
    pub registered_at: DateTime<Utc>,
    /// Last updated timestamp
    pub updated_at: DateTime<Utc>,
}

/// Registry operation statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RegistryStats {
    /// Total models registered
    pub total_models: usize,
    /// Total API calls made
    pub total_calls: u64,
    /// Total errors encountered
    pub total_errors: u64,
}

/// Trait for LLM-Registry operations
#[async_trait]
pub trait Registry: Send + Sync {
    /// Register a new model
    async fn register_model(
        &self,
        name: &str,
        version: &str,
        metadata: JsonValue,
    ) -> Result<String>;

    /// Get model metadata
    async fn get_model(&self, name: &str, version: &str) -> Result<ModelMetadata>;

    /// List all models
    async fn list_models(&self) -> Result<Vec<ModelMetadata>>;

    /// Update model metadata
    async fn update_model(&self, name: &str, version: &str, metadata: JsonValue) -> Result<()>;

    /// Delete a model
    async fn delete_model(&self, name: &str, version: &str) -> Result<()>;

    /// Get registry statistics
    async fn stats(&self) -> Result<RegistryStats>;
}

/// LLM-Registry HTTP client
#[derive(Debug, Clone)]
pub struct RegistryClient {
    config: RegistryConfig,
    client: Client,
}

impl RegistryClient {
    /// Create a new registry client
    pub fn new(config: RegistryConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_secs))
            .build()
            .map_err(|e| Error::ConfigError(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self { config, client })
    }

    /// Build the full URL for an endpoint
    fn build_url(&self, path: &str) -> String {
        format!("{}{}", self.config.base_url, path)
    }

    /// Check response status and convert errors
    fn check_response(&self, status: StatusCode, body: &str) -> Result<()> {
        match status {
            StatusCode::OK | StatusCode::CREATED | StatusCode::NO_CONTENT => Ok(()),
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => {
                Err(Error::AuthenticationError(body.to_string()))
            }
            StatusCode::NOT_FOUND => Err(Error::NotFound(body.to_string())),
            StatusCode::TOO_MANY_REQUESTS => Err(Error::RateLimitExceeded(body.to_string())),
            _ => Err(Error::InvalidResponse(format!(
                "HTTP {}: {}",
                status, body
            ))),
        }
    }
}

#[async_trait]
impl Registry for RegistryClient {
    async fn register_model(
        &self,
        name: &str,
        version: &str,
        metadata: JsonValue,
    ) -> Result<String> {
        let url = self.build_url("/api/v1/models");
        let payload = serde_json::json!({
            "name": name,
            "version": version,
            "metadata": metadata
        });

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .json(&payload)
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        self.check_response(status, &body)?;

        // Extract model ID from response
        let response_data: JsonValue = serde_json::from_str(&body)?;
        response_data["id"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| Error::InvalidResponse("Missing model ID in response".to_string()))
    }

    async fn get_model(&self, name: &str, version: &str) -> Result<ModelMetadata> {
        let url = self.build_url(&format!("/api/v1/models/{}/{}", name, version));

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        self.check_response(status, &body)?;

        serde_json::from_str(&body).map_err(Into::into)
    }

    async fn list_models(&self) -> Result<Vec<ModelMetadata>> {
        let url = self.build_url("/api/v1/models");

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        self.check_response(status, &body)?;

        let response_data: JsonValue = serde_json::from_str(&body)?;
        let models = response_data["models"]
            .as_array()
            .ok_or_else(|| Error::InvalidResponse("Expected models array".to_string()))?;

        models
            .iter()
            .map(|m| serde_json::from_value(m.clone()).map_err(Into::into))
            .collect()
    }

    async fn update_model(&self, name: &str, version: &str, metadata: JsonValue) -> Result<()> {
        let url = self.build_url(&format!("/api/v1/models/{}/{}", name, version));

        let response = self
            .client
            .put(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .json(&metadata)
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        self.check_response(status, &body)
    }

    async fn delete_model(&self, name: &str, version: &str) -> Result<()> {
        let url = self.build_url(&format!("/api/v1/models/{}/{}", name, version));

        let response = self
            .client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        self.check_response(status, &body)
    }

    async fn stats(&self) -> Result<RegistryStats> {
        let url = self.build_url("/api/v1/stats");

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        self.check_response(status, &body)?;

        serde_json::from_str(&body).map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_config() {
        let config = RegistryConfig::new("https://registry.example.com", "test-key")
            .with_timeout(60)
            .with_max_retries(5);

        assert_eq!(config.base_url, "https://registry.example.com");
        assert_eq!(config.api_key, "test-key");
        assert_eq!(config.timeout_secs, 60);
        assert_eq!(config.max_retries, 5);
    }

    #[test]
    fn test_registry_client_creation() {
        let config = RegistryConfig::new("https://registry.example.com", "test-key");
        let client = RegistryClient::new(config);
        assert!(client.is_ok());
    }
}
