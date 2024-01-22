#![forbid(unsafe_code)]

//! # Azure Functions
//!
//! Azure Functions handler written in safe Rust.
//!
//! ## Overview
//!
//! - Async function execution
//! - Support for common triggers
//! - Tracing to Application Insights
//!
//! ## Crate Feature Flags
//!
//! - `default`: Enables tracing and http.
//! - `tracing`: Enables tracing which connects to Application Insights.
//! - `http`: Enables HTTP function triggers.
//! - `timer`: Enables timer function triggers.
//! - `event-hub`: Enables event hub function triggers.

#[cfg(feature = "tracing")]
pub mod custom_tracing;
#[cfg(feature = "event-hub")]
pub mod event_hub;
#[cfg(feature = "http")]
pub mod http;
#[cfg(feature = "timer")]
pub mod timer;

use http_body_util::{Full, BodyExt};
use http_body_util::combinators::BoxBody;
use hyper::body::{Incoming, Bytes};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::net::TcpListener;
use std::collections::HashMap;
use std::error::Error;
use std::future::Future;
use std::net::SocketAddr;
#[cfg(feature = "tracing")]
use tracing::instrument::WithSubscriber;
#[cfg(feature = "tracing")]
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;

#[cfg(feature = "event-hub")]
use self::event_hub::EventHubPayload;
#[cfg(feature = "http")]
use self::http::HttpPayload;
#[cfg(feature = "timer")]
use self::timer::TimerPayload;

pub async fn azure_func_init<F, S, R>(handler: F, env: S) -> Result<(), Box<dyn Error>>
where
    F: Fn(FunctionPayload, S) -> R + std::marker::Send + 'static + Copy + std::marker::Sync,
    S: Clone + std::marker::Send + 'static,
    R: Future<Output = Result<FunctionsResponse, Box<dyn Error>>> + std::marker::Send + 'static,
{
    #[cfg(feature = "tracing")]
    let _ = tracing_log::LogTracer::init();

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match std::env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };
    let addr: SocketAddr = ([127, 0, 0, 1], port).into();
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (tcp, _) = listener.accept().await?;
        let io = TokioIo::new(tcp);

        let env = env.clone();

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new().serve_connection(io, service_fn(move |req| {
                request_handler(req, handler, env.clone())
            })).await {
                println!("Failed to serve connection: {:?}", err);
            }
        });
    }
}

fn log_error(error: String) -> Response<BoxBody<Bytes, hyper::Error>> {
    Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(full(json!({"Outputs":{"res":{"body":"An error occurred while processing the request, check the log for a detailed error message.","statusCode":"400","headers":{}}},"Logs":[error]}).to_string()))
        .unwrap()
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

async fn request_handler<F, S, R>(
    request: Request<Incoming>,
    handler: F,
    env: S,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error>
where
    F: Fn(FunctionPayload, S) -> R + std::marker::Send + 'static + Copy,
    S: Clone + std::marker::Send + 'static,
    R: Future<Output = Result<FunctionsResponse, Box<dyn Error>>> + std::marker::Send + 'static,
{
    let events = custom_tracing::CustomLayer::new(tracing::Level::INFO);
    #[cfg(feature = "tracing")]
    let subscriber = tracing_subscriber::registry().with(events.clone());

    let bytes = match request.collect().await {
        Ok(bytes) => bytes.to_bytes(),
        Err(error) => return Ok(log_error(format!("{:#?}", error))),
    };

    let vector: Vec<u8> = bytes.to_vec();
    let deserialize_request: FunctionPayload = match serde_json::from_slice(&vector) {
        Ok(deserialize_request) => deserialize_request,
        Err(error) => return Ok(log_error(format!("{:#?}", error))),
    };

    #[cfg(feature = "tracing")]
    let response = match handler(deserialize_request, env)
        .with_subscriber(subscriber)
        .await
    {
        Ok(mut response) => {
            response.logs = events.get();
            response
        }
        Err(error) => {
            let mut response = FunctionsResponse::default().body("An error occurred while processing the request, check the log for a detailed error message.");
            response.logs = events.get();
            response.logs.push(format!("{:#?}", error));
            response
        }
    };

    #[cfg(not(feature = "tracing"))]
    let response = match handler(deserialize_request, env).await {
        Ok(response) => response,
        Err(error) => return Ok(log_error(format!("{:#?}", error))),
    };

    let response_string: String = match serde_json::to_string(&response) {
        Ok(response_string) => response_string,
        Err(error) => return Ok(log_error(format!("{:#?}", error))),
    };

    let hyper_response = match Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(full(response_string))
    {
        Ok(hyper_response) => hyper_response,
        Err(error) => return Ok(log_error(format!("{:#?}", error))),
    };

    Ok(hyper_response)
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum FunctionPayload {
    #[cfg(feature = "http")]
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
    pub fn http(status_code: HttpStatusCode) -> Self {
        let mut response = FunctionsResponse::default();
        response.outputs.res.status_code = status_code;
        response
    }

    pub fn redirect(redirect_url: String) -> Self {
        let mut response = FunctionsResponse::default();
        response.outputs.res.status_code = HttpStatusCode::Found;
        response
            .outputs
            .res
            .headers
            .insert(String::from("Location"), redirect_url);
        response
    }

    pub fn body<S>(mut self, body: S) -> Self
    where
        S: Into<String>,
    {
        self.outputs.res.body = body.into();
        self
    }

    pub fn body_html<S>(mut self, value: S) -> Self
    where
        S: Into<String>,
    {
        self.outputs.res.body = value.into();
        self.outputs
            .res
            .headers
            .insert(String::from("Content-Type"), String::from("text/html"));
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
