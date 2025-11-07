//! Utility functions for error handling and retries

use crate::error::Result;
use std::future::Future;
use std::time::Duration;

/// Retry policy configuration
#[derive(Debug, Clone, Copy)]
pub struct RetryPolicy {
    /// Maximum number of attempts (including initial try)
    pub max_attempts: usize,
    /// Initial delay between retries
    pub initial_delay: Duration,
    /// Maximum delay between retries
    pub max_delay: Duration,
    /// Backoff multiplier
    pub backoff_multiplier: f64,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(5),
            backoff_multiplier: 2.0,
        }
    }
}

/// Retry an async operation with exponential backoff
pub async fn retry<F, Fut, T>(policy: RetryPolicy, mut operation: F) -> Result<T>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T>>,
{
    let mut attempt = 0;
    let mut delay = policy.initial_delay;

    loop {
        attempt += 1;

        match operation().await {
            Ok(value) => return Ok(value),
            Err(err) if attempt >= policy.max_attempts => return Err(err),
            Err(_) => {
                // Wait before retrying
                #[cfg(feature = "tokio")]
                tokio::time::sleep(delay).await;

                #[cfg(not(feature = "tokio"))]
                std::thread::sleep(delay);

                // Calculate next delay with exponential backoff
                delay = std::cmp::min(
                    Duration::from_millis(
                        (delay.as_millis() as f64 * policy.backoff_multiplier) as u64,
                    ),
                    policy.max_delay,
                );
            }
        }
    }
}

/// Simple retry function with default policy
pub async fn retry_default<F, Fut, T>(operation: F) -> Result<T>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T>>,
{
    retry(RetryPolicy::default(), operation).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retry_policy_default() {
        let policy = RetryPolicy::default();
        assert_eq!(policy.max_attempts, 3);
        assert_eq!(policy.initial_delay, Duration::from_millis(100));
    }
}
