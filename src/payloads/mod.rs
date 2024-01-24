use serde::Deserialize;
use self::http::HttpPayload;

#[cfg(feature = "event-hub")]
pub mod event_hub;
#[cfg(feature = "http")]
pub mod http;
#[cfg(feature = "timer")]
pub mod timer;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum FunctionPayload {
    #[cfg(feature = "http")]
    HttpData(http::HttpPayload),
    #[cfg(feature = "event-hub")]
    EventHubData(event_hub::EventHubPayload),
    #[cfg(feature = "timer")]
    TimerData(timer::TimerPayload),
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum InputBinding {
    Blob(String),
}

pub trait FromPayload {
    fn from_payload(payload: Vec<u8>) -> Result<Self, serde_json::Error> where Self: Sized;
    fn method_name(&self) -> String;
}

impl FromPayload for HttpPayload {
    fn from_payload(payload: Vec<u8>) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(&payload)
    }

    fn method_name(&self) -> String {
        self.method_name()
    }
}