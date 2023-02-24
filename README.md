<div>
  <div align="center" style="display: block; text-align: center;">
    <img src="https://raw.githubusercontent.com/daniel-larsen/azure-functions-rust/main/assets/azurefunctions-rust.svg" height="150" />
  </div>
</div>

> This project is not an official Microsoft product.

## Why Azure Functions and Rust

Reduced cold start latency

- Zip deployed app is less than 2MB which reduces the time needed to copy the app from storage.
- Entire app is precompiled, no waiting for interpreter or JIT compiler to make machine code.

Rust benefits

- Rust runs fast and is efficient with memory which reduces execution time and cost.
- Strong type system with guaranteed memory and thread safety.

Azure Functions benefits

- Use Function triggers and bindings to write less boilerplate code.
- Use App Service auth to get simple authentication and authorization.

## Features

- Async function execution
- Support for common triggers
- Log to Application Insights

## Installation

1. Add azure-functions-rust to your Cargo.toml file. Make sure tokio and hyper are all included.

```toml
[dependencies]
tokio = { version = "1", features = ["rt", "macros", "rt-multi-thread"] }
azure_functions = { git = "https://github.com/daniel-larsen/azure-functions-rust" }
tracing = "0.1"
log = "0.4"

```

2. Initalize the handler from your main.rs file.

```rust
use azure_functions::{
    azure_func_init, http::HttpPayload, FunctionPayload, FunctionsResponse, HttpStatusCode,
};
use std::error::Error;

fn my_http_func(payload: HttpPayload) -> Result<FunctionsResponse, Box<dyn Error>> {
    tracing::info!("This will be logged to Application Insights");
    log::info!("This will also be logged to Application Insights");
    let response =
        FunctionsResponse::http(HttpStatusCode::Ok).body(payload.metadata.sys.utc_now.to_string());
    Ok(response)
}

async fn handler(
    payload: FunctionPayload,
    _env: Environment,
) -> Result<FunctionsResponse, Box<dyn Error>> {
    match payload {
        FunctionPayload::HttpData(payload) => match payload.method_name() {
            "MyHttpFunc" => my_http_func(payload),
            _ => Ok(FunctionsResponse::http(HttpStatusCode::NotFound).body("path not found")),
        },
    }
}

#[derive(Debug, Clone)]
pub struct Environment {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let environment = Environment {}; // used to pass clients to the function handlers

    azure_func_init(handler, environment).await;
    Ok(())
}

```

> Credits \
> Azure and the Azure Functions logo are property of the Microsoft Corporation.\
> Rust and the Rust logo are property of the Rust Foundation.
