# Rust Handler for Azure Functions (Work in Progress)

This repository contains an Azure Functions handler for Rust.

## Installation

1. Add azure-functions-rust to your Cargo.toml file. Make sure tokio and hyper are all included.

```toml
[dependencies]
tokio = { version = "1", features = ["rt", "macros", "rt-multi-thread"] }
hyper = { version = "0.14", features = ["full"] }
azure_functions = { git = "https://github.com/daniel-larsen/azure-functions-rust", branch = "main" }

```
2. Initalize the handler from your main.rs file. 

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let keyvault_url = std::env::args()
        .nth(1)
        .expect("Missing KEYVAULT_URL environment variable.");

    let creds = DefaultAzureCredential::default();
    let keyvault_client = CertificateClient::new(keyvault_url.as_str(), Arc::new(creds))?;

    let environment = Environment {
        certificate_client: keyvault_client,
    };

    azure_func_init(environment).await;
    Ok(())
}

```