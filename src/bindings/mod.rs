use crate::payloads::http::HttpMethod;
use self::http::HttpBindingParams;

#[cfg(feature = "event-hub")]
pub mod event_hub;
#[cfg(feature = "http")]
pub mod http;
#[cfg(feature = "timer")]
pub mod timer;

pub enum InputBinding {
    #[cfg(feature = "http")]
    Http(http::HttpBindingParams),
    #[cfg(feature = "event-hub")]
    EventHub(),
    #[cfg(feature = "timer")]
    Timer(),
}

impl InputBinding {
    pub fn http<S>(route: S, methods: Vec<HttpMethod>) -> Self
    where S: Into<String>
    {
        let param = HttpBindingParams{route: route.into(), methods};
        Self::Http(param)
    }
}