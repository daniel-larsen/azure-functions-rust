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

pub mod payloads;
pub mod bindings;
pub mod utils;
pub mod response;

use http_body_util::{Full, BodyExt};
use http_body_util::combinators::BoxBody;
use hyper::body::{Incoming, Bytes};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use payloads::FromPayload;
use serde_json::json;
use tokio::net::TcpListener;
use response::FunctionsResponse;
use bindings::InputBinding;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::future::Future;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::Arc;
#[cfg(feature = "tracing")]
use tracing::instrument::WithSubscriber;
#[cfg(feature = "tracing")]
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;

pub struct AzureFuncHandler<F, P, S, R> where
    F: Fn(P, S) -> R + std::marker::Send + 'static + Copy + std::marker::Sync,
    P: FromPayload + 'static + std::marker::Sync + std::marker::Send,
    S: Clone + std::marker::Send + 'static + std::marker::Sync,
    R: Future<Output = Result<FunctionsResponse, Box<dyn Error>>> + std::marker::Send + 'static,
{
    inner: Arc<AzureFuncHandlerInner<F, P, S, R>>,
}

struct AzureFuncHandlerInner<F, P, S, R> 
where
    F: Fn(P, S) -> R + std::marker::Send + 'static + Copy + std::marker::Sync,
    P: FromPayload + 'static + std::marker::Sync + std::marker::Send,
    S: Clone + std::marker::Send + 'static + std::marker::Sync,
    R: Future<Output = Result<FunctionsResponse, Box<dyn Error>>> + std::marker::Send + 'static,
{
    functions: HashMap<String, (F, InputBinding)>,
    env: S,
    phantom: PhantomData<P>
}

impl<F, P, S, R> Clone for AzureFuncHandler<F, P, S, R> 
where
    F: Fn(P, S) -> R + std::marker::Send + 'static + Copy + std::marker::Sync,
    P: FromPayload + 'static + std::marker::Sync + std::marker::Send,
    S: Clone + std::marker::Send + 'static + std::marker::Sync,
    R: Future<Output = Result<FunctionsResponse, Box<dyn Error>>> + std::marker::Send + 'static,
{
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}


impl<F, P, S, R> AzureFuncHandler<F, P, S, R> 
where
    F: Fn(P, S) -> R + std::marker::Send + 'static + Copy + std::marker::Sync,
    P: FromPayload + 'static + std::marker::Sync + std::marker::Send,
    S: Clone + std::marker::Send + 'static + std::marker::Sync,
    R: Future<Output = Result<FunctionsResponse, Box<dyn Error>>> + std::marker::Send + 'static,
{
    pub fn new(env: S) -> Self {
        Self {
            inner: Arc::new(AzureFuncHandlerInner {
                functions: HashMap::new(),
                env,
                phantom: PhantomData
            })
        }
    }

    pub async fn start(self) -> Result<(), Box<dyn Error>>
    {
        #[cfg(feature = "tracing")]
        let _ = tracing_log::LogTracer::init();

        let port: u16 = match std::env::var("FUNCTIONS_CUSTOMHANDLER_PORT") {
            Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
            Err(_) => 3000,
        };
        let addr: SocketAddr = ([127, 0, 0, 1], port).into();
        let listener = TcpListener::bind(addr).await?;

        loop {
            let (tcp, _) = listener.accept().await?;
            let io = TokioIo::new(tcp);

            let handlers = self.clone();

            tokio::task::spawn(async move {
                if let Err(err) = http1::Builder::new().serve_connection(io, service_fn(move |req| {                    
                    request_handler(req, handlers.clone())
                })).await {
                    println!("Failed to serve connection: {:?}", err);
                }
            });
        }
    }

    pub fn trigger<N>(self, name: N, handler: F, trigger: InputBinding) -> Self
        where N: Into<String>
    {
        match Arc::try_unwrap(self.inner) {
            Ok(mut inner) => {
                inner.functions.insert(name.into(), (handler, trigger));

                Self {
                    inner: Arc::new(inner)
                }
            }
            Err(_) => panic!("could not add endpoint")
        }
    }

    pub fn update_bindings(self) -> Result<(), Box<dyn Error>>
    {
        for function in self.inner.functions.iter() {
            fs::create_dir(function.0)?;
            let json = match &function.1.1 {
                InputBinding::Http(binding) => binding.to_string()
            };
            fs::write(format!("./{}/function.json", function.0), json)?;
        }
        Ok(())
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

async fn request_handler<F, P, S, R>(
    request: Request<Incoming>,
    handlers: AzureFuncHandler<F, P, S, R>
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error>
where
    F: Fn(P, S) -> R + std::marker::Send + 'static + Copy + std::marker::Sync,
    P: FromPayload + 'static + std::marker::Sync + std::marker::Send,
    S: Clone + std::marker::Send + 'static + std::marker::Sync,
    R: Future<Output = Result<FunctionsResponse, Box<dyn Error>>> + std::marker::Send + 'static,
{
    let events = utils::tracing::CustomLayer::new(tracing::Level::INFO);
    #[cfg(feature = "tracing")]
    let subscriber = tracing_subscriber::registry().with(events.clone());

    let bytes = match request.collect().await {
        Ok(bytes) => bytes.to_bytes(),
        Err(error) => return Ok(log_error(format!("{:#?}", error))),
    };

    let payload = P::from_payload(bytes.to_vec()).unwrap();
    let handler = handlers.inner.functions.get(&payload.method_name()).unwrap().0;

    #[cfg(feature = "tracing")]
    let response = match handler(payload, handlers.inner.env.clone())
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

