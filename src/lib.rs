#[cfg(feature = "event-hub")]
pub mod event_hub;
pub mod http;
#[cfg(feature = "timer")]
pub mod timer;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::error::Error;
use std::future::Future;

#[cfg(feature = "event-hub")]
use self::event_hub::EventHubPayload;
use self::http::HttpPayload;
#[cfg(feature = "timer")]
use self::timer::TimerPayload;

pub async fn azure_func_init<F, S, R>(handler: F, env: S)
where
    F: Fn(FunctionPayload, S) -> R + std::marker::Send + 'static + Copy + std::marker::Sync,
    S: Clone + std::marker::Send + 'static,
    R: Future<Output = Result<FunctionsResponse, Box<dyn Error>>> + std::marker::Send + 'static,
{
    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match std::env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    let addr = ([127, 0, 0, 1], port).into();
    let service = make_service_fn(move |_| {
        let env = env.clone();
        async move {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                request_handler(req, handler, env.clone())
            }))
        }
    });
    let server = Server::bind(&addr).serve(service);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

fn log_error(error: String) -> Response<Body> {
    Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(Body::from(json!({"Outputs":{"res":{"body":"An error occurred while processing the request, check the log for a detailed error message.","statusCode":"400","headers":{}}},"Logs":[error]}).to_string()))
        .unwrap()
}

// This is our service handler. It receives a Request, routes on its
// path, and returns a Future of a Response.
async fn request_handler<F, S, R>(
    request: Request<Body>,
    handler: F,
    env: S,
) -> Result<Response<Body>, hyper::Error>
where
    F: Fn(FunctionPayload, S) -> R + std::marker::Send + 'static + Copy,
    S: Clone + std::marker::Send + 'static,
    R: Future<Output = Result<FunctionsResponse, Box<dyn Error>>> + std::marker::Send + 'static,
{
    let bytes = match hyper::body::to_bytes(request.into_body()).await {
        Ok(bytes) => bytes,
        Err(error) => return Ok(log_error(format!("{:#?}", error))),
    };

    let vector: Vec<u8> = bytes.to_vec();
    // println!("{:?}", std::str::from_utf8(&vector).unwrap());
    let deserialize_request: FunctionPayload = match serde_json::from_slice(&vector) {
        Ok(deserialize_request) => deserialize_request,
        Err(error) => return Ok(log_error(format!("{:#?}", error))),
    };

    let response = match handler(deserialize_request, env).await {
        Ok(response) => response,
        Err(error) => return Ok(log_error(format!("{:#?}", error))),
    };

    let response_string: String = match serde_json::to_string(&response) {
        Ok(response_string) => response_string,
        Err(error) => return Ok(log_error(format!("{:#?}", error))),
    };

    //println!("{}", response_string);
    let hyper_response = match Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(Body::from(response_string))
    {
        Ok(hyper_response) => hyper_response,
        Err(error) => return Ok(log_error(format!("{:#?}", error))),
    };

    Ok(hyper_response)
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum FunctionPayload {
    HttpData(HttpPayload),
    #[cfg(feature = "event-hub")]
    EventHubData(EventHubPayload),
    #[cfg(feature = "timer")]
    TimerData(TimerPayload),
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum InputBinding {
    Blob(String),
}

#[derive(Default, Serialize)]
pub struct FunctionsResponse {
    #[serde(rename = "Outputs")]
    pub outputs: FunctionsOutput,
    #[serde(rename = "Logs")]
    pub logs: Vec<String>,
}

impl FunctionsResponse {
    pub fn logs_new<T>(&mut self, message: T)
    where
        T: Into<String>,
    {
        self.logs.push(message.into());
    }

    pub fn http(status_code: HttpStatusCode) -> Self {
        let mut response = FunctionsResponse::default();
        response.outputs.res.status_code = status_code;
        response
    }

    pub fn body<S>(mut self, body: S) -> Self
    where
        S: Into<String>,
    {
        self.outputs.res.body = body.into();
        self
    }

    pub fn body_json<T>(mut self, value: &T) -> Result<Self, serde_json::Error>
    where
        T: ?Sized + Serialize,
    {
        self.outputs.res.body = serde_json::to_string(value)?;
        self.outputs.res.headers.insert(
            String::from("Content-Type"),
            String::from("application/json"),
        );
        Ok(self)
    }
}

#[derive(Default, Serialize)]
pub struct FunctionsOutput {
    pub res: FunctionsResponseData,
}

#[derive(Default, Serialize)]
pub enum HttpStatusCode {
    #[serde(rename = "200")]
    Ok,
    #[serde(rename = "302")]
    Found,
    #[default]
    #[serde(rename = "400")]
    BadRequest,
    #[serde(rename = "401")]
    Unauthorized,
    #[serde(rename = "404")]
    NotFound,
}

#[derive(Default, Serialize)]
pub struct FunctionsResponseData {
    pub body: String,
    #[serde(rename = "statusCode")]
    pub status_code: HttpStatusCode,
    pub headers: HashMap<String, String>,
}
