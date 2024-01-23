use crate::payloads::http::HttpMethod;

pub struct HttpBindingParams {
    pub route: String,
    pub methods: Vec<HttpMethod>
}

impl HttpBindingParams {
    pub fn new<S>(route: S, methods: Vec<HttpMethod>) -> Self
    where S: Into<String>
    {
        Self{ route: route.into(), methods}
    }
}

impl ToString for HttpBindingParams {
    fn to_string(&self) -> String {
        format!(r#"{{"bindings":[{{"authLevel":"anonymous","type":"httpTrigger","direction":"in","route":"{}","name":"req","methods":["get","post","patch"]}},{{"type": "http","direction": "out","name": "res"}}]}}"#, self.route)
    }
}