use azure_functions::{
    azure_func_init, event_hub::EventHubPayload, FunctionPayload, FunctionsResponse, HttpStatusCode,
};
use std::error::Error;

fn my_event_hub_func(payload: EventHubPayload) -> Result<FunctionsResponse, Box<dyn Error>> {
    // both tracing and log messages are captured
    log::info!("This will be logged to Application Insights");
    log::info!("{}", payload.data);

    Ok(FunctionsResponse::default())
}

async fn handler(
    payload: FunctionPayload,
    _env: Environment,
) -> Result<FunctionsResponse, Box<dyn Error>> {
    match payload {
        FunctionPayload::EventHubData(payload) => match payload.method_name() {
            "MyEventHubFunc" => my_event_hub_func(payload),
            _ => Ok(FunctionsResponse::http(HttpStatusCode::NotFound).body("path not found")),
        },
    }
}

#[derive(Clone)]
pub struct Environment {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let environment = Environment {}; // used to pass clients to the function handlers

    azure_func_init(handler, environment).await;
    Ok(())
}
