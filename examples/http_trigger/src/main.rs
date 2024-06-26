use azure_functions::{
    azure_func_init, http::HttpPayload, FunctionPayload, FunctionsResponse, HttpStatusCode,
};
use std::error::Error;

fn my_http_func(payload: HttpPayload) -> Result<FunctionsResponse, Box<dyn Error>> {
    // both tracing and log messages are captured
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
