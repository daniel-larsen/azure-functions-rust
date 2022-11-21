use azure_functions::{azure_func_init, FunctionPayload, FunctionsResponse, HttpStatusCode};
use hyper::{Body, Request, Response};

async fn process_request(
    payload: FunctionPayload,
    response: &mut FunctionsResponse,
    _env: Environment,
) {
    match payload {
        FunctionPayload::HttpData(payload) => match payload.metadata.sys.method_name.as_str() {
            "HttpTrigger" => {
                response.logs_new("This message will be visible in Application Insights");
                response.outputs.res.body = payload.metadata.sys.utc_now.to_string();
                response.outputs.res.status_code = HttpStatusCode::Ok;
            }
            _ => response.outputs.res.body = "path not found".to_string(),
        },
        FunctionPayload::EventHubData(_payload) => {}

        FunctionPayload::TimerData(_payload) => {}
    }
}

// This is our service handler. It receives a Request, routes on its
// path, and returns a Future of a Response.
async fn request_handler(
    request: Request<Body>,
    env: Environment,
) -> Result<Response<Body>, hyper::Error> {
    let bytes = hyper::body::to_bytes(request.into_body()).await.unwrap();
    let vector: Vec<u8> = bytes.to_vec();
    // println!("{:?}", std::str::from_utf8(&vector).unwrap());
    let deserialize_request: Result<FunctionPayload, serde_json::Error> =
        serde_json::from_slice(&vector);

    let mut response: FunctionsResponse = Default::default();

    if deserialize_request.is_err() {
        response.outputs.res.body = deserialize_request.err().unwrap().to_string();
    } else {
        process_request(deserialize_request.unwrap(), &mut response, env).await;
    }

    let response_string: String = serde_json::to_string(&response).unwrap();
    // println!("{}", response_string);
    let hyper_response = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(Body::from(response_string))
        .unwrap();

    Ok(hyper_response)
}

#[derive(Debug, Clone)]
pub struct Environment {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let environment = Environment {}; // used to pass clients to the function handlers

    azure_func_init(request_handler, environment).await;
    Ok(())
}
