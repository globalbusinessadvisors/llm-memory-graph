# llm-memory-graph-integrations

Integration clients for external services used by the LLM Memory Graph system.

## Features

- **LLM-Registry Client**: Interact with model registry for metadata tracking
- **Data-Vault Client**: Archive and retrieve session data for long-term storage
- Async/await support with tokio
- Comprehensive error handling
- Retry logic and timeout configuration

## Installation

```toml
[dependencies]
llm-memory-graph-integrations = "0.1.0"
```

## Usage

### LLM-Registry

```rust
use llm_memory_graph_integrations::registry::{RegistryClient, RegistryConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = RegistryConfig::new(
        "https://registry.example.com",
        "your-api-key"
    );

    let client = RegistryClient::new(config)?;

    // Register a model
    let model_id = client.register_model(
        "gpt-4",
        "1.0.0",
        serde_json::json!({
            "provider": "OpenAI",
            "context_window": 8192
        })
    ).await?;

    // Get model metadata
    let metadata = client.get_model("gpt-4", "1.0.0").await?;
    println!("Model: {:?}", metadata);

    Ok(())
}
```

### Data-Vault

```rust
use llm_memory_graph_integrations::vault::{VaultClient, VaultConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = VaultConfig::new(
        "https://vault.example.com",
        "your-api-key"
    );

    let client = VaultClient::new(config)?;

    // Archive a session
    let session_data = b"session data here";
    let archive = client.archive_session("session-123", session_data).await?;
    println!("Archived: {:?}", archive);

    // Retrieve a session
    let data = client.retrieve_session("session-123").await?;

    Ok(())
}
```

## Configuration

Both clients support configuration options:

- `timeout_secs`: Request timeout (default: 30 seconds)
- `max_retries`: Maximum retry attempts (default: 3)
- Vault-specific: `enable_compression` (default: true)

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
