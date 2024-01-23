use serde::Deserialize;

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