use azure_functions::{azure_func_init, FunctionPayload, FunctionsResponse, HttpStatusCode};
use std::fmt::Error;

async fn handler(payload: FunctionPayload, _env: Environment) -> Result<FunctionsResponse, Error> {
    let mut response = FunctionsResponse::default();

    match payload {
        FunctionPayload::HttpData(payload) => match payload.method_name() {
            "HttpTrigger" => {
                response.logs_new("This message will be visible in Application Insights");
                response.outputs.res.body = payload.metadata.sys.utc_now.to_string();
                response.outputs.res.status_code = HttpStatusCode::Ok;
            }
            "HttpTriggerAuth" => {
                #[cfg(not(debug_assertions))]
                require_auth_redirect!(payload.data.req, response);
                response.logs_new("This message will be visible in Application Insights");
                response.outputs.res.body = payload.metadata.sys.utc_now.to_string();
                response.outputs.res.status_code = HttpStatusCode::Ok;
            }
            _ => response.outputs.res.body = "path not found".to_string(),
        },
        FunctionPayload::EventHubData(_payload) => {}

        FunctionPayload::TimerData(_payload) => {}
    };
    return Ok(response);
}

#[derive(Debug, Clone)]
pub struct Environment {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let environment = Environment {}; // used to pass clients to the function handlers

    azure_func_init(handler, environment).await;
    Ok(())
}
