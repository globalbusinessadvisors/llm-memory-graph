//! Data-Vault integration client
//!
//! Provides a client for interacting with the Data-Vault service,
//! which provides long-term archival and retrieval of session data.

use crate::{Error, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Configuration for Data-Vault client
#[derive(Debug, Clone)]
pub struct VaultConfig {
    /// Base URL of the vault service
    pub base_url: String,
    /// API key for authentication
    pub api_key: String,
    /// Request timeout in seconds
    pub timeout_secs: u64,
    /// Maximum retry attempts
    pub max_retries: usize,
    /// Enable compression for archived data
    pub enable_compression: bool,
}

impl VaultConfig {
    /// Create a new vault configuration
    pub fn new(base_url: impl Into<String>, api_key: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            api_key: api_key.into(),
            timeout_secs: 30,
            max_retries: 3,
            enable_compression: true,
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

    /// Enable or disable compression
    pub fn with_compression(mut self, enable: bool) -> Self {
        self.enable_compression = enable;
        self
    }
}

/// Metadata for an archived session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchivedSession {
    /// Session ID
    pub session_id: String,
    /// Size of archived data in bytes
    pub size_bytes: usize,
    /// Whether data is compressed
    pub compressed: bool,
    /// When the session was archived
    pub archived_at: DateTime<Utc>,
    /// Archive storage location
    pub storage_location: String,
}

/// Vault operation statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VaultStats {
    /// Total sessions archived
    pub total_archives: u64,
    /// Total sessions retrieved
    pub total_retrievals: u64,
    /// Total bytes stored
    pub total_bytes: u64,
    /// Total errors encountered
    pub total_errors: u64,
}

/// Trait for Data-Vault operations
#[async_trait]
pub trait Vault: Send + Sync {
    /// Archive a session
    async fn archive_session(&self, session_id: &str, data: &[u8]) -> Result<ArchivedSession>;

    /// Retrieve an archived session
    async fn retrieve_session(&self, session_id: &str) -> Result<Vec<u8>>;

    /// Get archived session metadata
    async fn get_session_metadata(&self, session_id: &str) -> Result<ArchivedSession>;

    /// List all archived sessions
    async fn list_sessions(&self) -> Result<Vec<ArchivedSession>>;

    /// Delete an archived session
    async fn delete_session(&self, session_id: &str) -> Result<()>;

    /// Get vault statistics
    async fn stats(&self) -> Result<VaultStats>;
}

/// Data-Vault HTTP client
#[derive(Debug, Clone)]
pub struct VaultClient {
    config: VaultConfig,
    client: Client,
}

impl VaultClient {
    /// Create a new vault client
    pub fn new(config: VaultConfig) -> Result<Self> {
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
impl Vault for VaultClient {
    async fn archive_session(&self, session_id: &str, data: &[u8]) -> Result<ArchivedSession> {
        let url = self.build_url(&format!("/api/v1/sessions/{}/archive", session_id));

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/octet-stream")
            .header(
                "X-Compression-Enabled",
                self.config.enable_compression.to_string(),
            )
            .body(data.to_vec())
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        self.check_response(status, &body)?;

        serde_json::from_str(&body).map_err(Into::into)
    }

    async fn retrieve_session(&self, session_id: &str) -> Result<Vec<u8>> {
        let url = self.build_url(&format!("/api/v1/sessions/{}/data", session_id));

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .send()
            .await?;

        let status = response.status();

        if !status.is_success() {
            let body = response.text().await?;
            self.check_response(status, &body)?;
            // If check_response doesn't return an error, unreachable
            unreachable!()
        }

        response.bytes().await.map(|b| b.to_vec()).map_err(Into::into)
    }

    async fn get_session_metadata(&self, session_id: &str) -> Result<ArchivedSession> {
        let url = self.build_url(&format!("/api/v1/sessions/{}", session_id));

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

    async fn list_sessions(&self) -> Result<Vec<ArchivedSession>> {
        let url = self.build_url("/api/v1/sessions");

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .send()
            .await?;

        let status = response.status();
        let body = response.text().await?;

        self.check_response(status, &body)?;

        let response_data: serde_json::Value = serde_json::from_str(&body)?;
        let sessions = response_data["sessions"]
            .as_array()
            .ok_or_else(|| Error::InvalidResponse("Expected sessions array".to_string()))?;

        sessions
            .iter()
            .map(|s| serde_json::from_value(s.clone()).map_err(Into::into))
            .collect()
    }

    async fn delete_session(&self, session_id: &str) -> Result<()> {
        let url = self.build_url(&format!("/api/v1/sessions/{}", session_id));

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

    async fn stats(&self) -> Result<VaultStats> {
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
    fn test_vault_config() {
        let config = VaultConfig::new("https://vault.example.com", "test-key")
            .with_timeout(60)
            .with_max_retries(5)
            .with_compression(false);

        assert_eq!(config.base_url, "https://vault.example.com");
        assert_eq!(config.api_key, "test-key");
        assert_eq!(config.timeout_secs, 60);
        assert_eq!(config.max_retries, 5);
        assert!(!config.enable_compression);
    }

    #[test]
    fn test_vault_client_creation() {
        let config = VaultConfig::new("https://vault.example.com", "test-key");
        let client = VaultClient::new(config);
        assert!(client.is_ok());
    }
}
