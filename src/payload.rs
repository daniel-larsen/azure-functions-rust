use serde::Deserialize;
use crate::triggers;

#[cfg(feature = "event-hub")]
use triggers::event_hub::EventHubPayload;
#[cfg(feature = "http")]
use triggers::http::HttpPayload;
#[cfg(feature = "timer")]
use triggers::timer::TimerPayload;

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

// trait FromPayload {
//     fn from_payload(payload: &FunctionPayload) -> Self;
// }

// pub struct Param(pub String);

// impl FromPayload for Param {
//     fn from_payload(payload: &FunctionPayload) -> Self {
//         Param(payload.param.clone())
//     }
// }

// trait Handler<T> {
//     fn call(self, payload: FunctionPayload);
// }

// impl<F, T> Handler<T> for F
// where
//     F: Fn(T),
//     T: FromPayload,
// {
//     fn call(self, payload: FunctionPayload) {
//         (self)(T::from_payload(&payload));
//     }
// }