pub mod event_hub;
pub mod http;
pub mod timer;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::future::Future;

use self::event_hub::EventHubPayload;
use self::http::HttpPayload;
use self::timer::TimerPayload;

pub async fn azure_func_init<F, E, S>(handler: F, env: E)
where
    F: Fn(Request<Body>, E) -> S + std::marker::Send + 'static + Copy,
    E: Clone + std::marker::Send + 'static,
    S: Future<Output = Result<Response<Body>, hyper::Error>> + std::marker::Send + 'static,
{
    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match std::env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    let addr = ([127, 0, 0, 1], port).into();
    let service = make_service_fn(move |_| {
        let env = env.clone();
        async move { Ok::<_, hyper::Error>(service_fn(move |req| handler(req, env.clone()))) }
    });
    let server = Server::bind(&addr).serve(service);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum FunctionPayload {
    HttpData(HttpPayload),
    EventHubData(EventHubPayload),
    TimerData(TimerPayload),
}

// impl FunctionPayload {
//     pub fn method_name(&self) {
//         return self.metadata.sys.method_name;
//     }
// }

#[derive(Serialize)]
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
}

impl Default for FunctionsResponse {
    fn default() -> FunctionsResponse {
        FunctionsResponse {
            outputs: FunctionsOutput::default(),
            logs: vec![],
        }
    }
}

#[derive(Serialize)]
pub struct FunctionsOutput {
    pub res: FunctionsResponseData,
}

impl Default for FunctionsOutput {
    fn default() -> FunctionsOutput {
        FunctionsOutput {
            res: FunctionsResponseData::default(),
        }
    }
}

#[derive(Serialize)]
pub enum HttpStatusCode {
    #[serde(rename = "200")]
    Ok,
    #[serde(rename = "302")]
    Found,
    #[serde(rename = "400")]
    BadRequest,
    #[serde(rename = "401")]
    Unauthorized,
    #[serde(rename = "404")]
    NotFound,
}

#[derive(Serialize)]
pub struct FunctionsResponseData {
    pub body: String,
    #[serde(rename = "statusCode")]
    pub status_code: HttpStatusCode,
    pub headers: HashMap<String, String>,
}

impl Default for FunctionsResponseData {
    fn default() -> FunctionsResponseData {
        FunctionsResponseData {
            body: "".to_string(),
            status_code: HttpStatusCode::BadRequest,
            headers: HashMap::new(),
        }
    }
}
