pub mod event_hub;
pub mod http;
pub mod timer;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use self::event_hub::EventHubPayload;
use self::http::HttpPayload;
use self::timer::TimerPayload;

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
