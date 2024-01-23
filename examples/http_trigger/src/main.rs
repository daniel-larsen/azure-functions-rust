use azure_functions::{
    AzureFuncHandler, FunctionPayload, FunctionsResponse, HttpStatusCode
};
use std::error::Error;

async fn my_http_func(payload: FunctionPayload, env: Environment) -> Result<FunctionsResponse, Box<dyn Error>> {
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
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let environment = Environment {}; // used to pass clients to the function handlers

    let handler = AzureFuncHandler::new(environment)
    .add("MyHttpFunc", my_http_func);
    handler.start().await;

    Ok(())
}