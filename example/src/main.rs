use azure_functions::{http::HttpMethod, FunctionPayload, FunctionsResponse, HttpStatusCode};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use serde_json::Error;

async fn process_request(
    payload: FunctionPayload,
    response: &mut FunctionsResponse,
    env: Environment,
) {
    match payload {
        FunctionPayload::HttpData(payload) => match payload.metadata.sys.method_name.as_str() {
            "HttpTrigger" => {
                response.outputs.res.body = "Success".to_string();
                response.outputs.res.status_code = HttpStatusCode::Ok;
            }
            _ => response.outputs.res.body = "path not found".to_string(),
        },
        FunctionPayload::EventHubData(payload) => {}

        FunctionPayload::TimerData(payload) => {}
        _ => response.outputs.res.body = "payload type not found".to_string(),
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
    let deserialize_request: Result<FunctionPayload, Error> = serde_json::from_slice(&vector);

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

async fn azure_func_init(env: Environment) {
    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match std::env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    let addr = ([127, 0, 0, 1], port).into();
    let service = make_service_fn(move |_| {
        let env = env.clone();
        async move { Ok::<_, hyper::Error>(service_fn(move |req| request_handler(req, env.clone()))) }
    });
    let server = Server::bind(&addr).serve(service);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

#[derive(Debug, Clone)]
pub struct Environment {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let environment = Environment {}; // used to pass clients to the function handlers

    azure_func_init(environment).await;
    Ok(())
}
