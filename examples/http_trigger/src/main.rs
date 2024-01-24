use std::error::Error;
use azure_functions::{bindings::InputBinding, payloads::http::{HttpMethod::{Get, Post}, HttpPayload}, response::{FunctionsResponse, HttpStatusCode}, AzureFuncHandler};

async fn my_http_func(payload: HttpPayload, env: Environment) -> Result<FunctionsResponse, Box<dyn Error>> {
    // both tracing and log messages are captured
    tracing::info!("This will be logged to Application Insights");
    log::info!("This will also be logged to Application Insights");

    let response =
        FunctionsResponse::http(HttpStatusCode::Ok).body("Testing");
    Ok(response)
}

#[derive(Debug, Clone)]
pub struct Environment {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let environment = Environment {}; // used to pass clients to the function handlers

    let handler = AzureFuncHandler::new(environment)
    .trigger("MyHttpFunc", my_http_func, InputBinding::http("MyHttpFunc", vec![Get, Post]));

    match std::env::args().nth(1) {
        Some(arg) if arg == "update" => handler.update_bindings()?,
        _ => handler.start().await?
    };

    Ok(())
}