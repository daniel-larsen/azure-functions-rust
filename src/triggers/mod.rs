use crate::payloads::http::HttpMethod;

use self::http::HttpTriggerParams;

#[cfg(feature = "event-hub")]
pub mod event_hub;
#[cfg(feature = "http")]
pub mod http;
#[cfg(feature = "timer")]
pub mod timer;

pub enum Trigger {
    #[cfg(feature = "http")]
    Http(http::HttpTriggerParams),
    #[cfg(feature = "event-hub")]
    EventHub(),
    #[cfg(feature = "timer")]
    Timer(),
}

impl Trigger {
    pub fn http<S>(route: S, methods: Vec<HttpMethod>) -> Self
    where S: Into<String>
    {
        let param = HttpTriggerParams{route: route.into(), methods};
        Self::Http(param)
    }
}