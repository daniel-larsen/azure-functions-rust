use crate::payloads::http::HttpMethod;

pub struct HttpTriggerParams {
    pub route: String,
    pub methods: Vec<HttpMethod>
}

impl HttpTriggerParams {
    pub fn new<S>(route: S, methods: Vec<HttpMethod>) -> Self
    where S: Into<String>
    {
        Self{ route: route.into(), methods}
    }
}