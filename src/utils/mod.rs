use std::collections::HashMap;

use serde::Serialize;

#[cfg(feature = "tracing")]
pub mod tracing;

#[derive(Default, Serialize)]
pub struct FunctionsResponse {
    #[serde(rename = "Outputs")]
    pub outputs: FunctionsOutput,
    #[serde(rename = "Logs")]
    pub logs: Vec<String>,
}

impl FunctionsResponse {
    pub fn http(status_code: HttpStatusCode) -> Self {
        let mut response = FunctionsResponse::default();
        response.outputs.res.status_code = status_code;
        response
    }

    pub fn redirect(redirect_url: String) -> Self {
        let mut response = FunctionsResponse::default();
        response.outputs.res.status_code = HttpStatusCode::Found;
        response
            .outputs
            .res
            .headers
            .insert(String::from("Location"), redirect_url);
        response
    }

    pub fn body<S>(mut self, body: S) -> Self
    where
        S: Into<String>,
    {
        self.outputs.res.body = body.into();
        self
    }

    pub fn body_html<S>(mut self, value: S) -> Self
    where
        S: Into<String>,
    {
        self.outputs.res.body = value.into();
        self.outputs
            .res
            .headers
            .insert(String::from("Content-Type"), String::from("text/html"));
        self
    }

    pub fn body_json<T>(mut self, value: &T) -> Result<Self, serde_json::Error>
    where
        T: ?Sized + Serialize,
    {
        self.outputs.res.body = serde_json::to_string(value)?;
        self.outputs.res.headers.insert(
            String::from("Content-Type"),
            String::from("application/json"),
        );
        Ok(self)
    }
}

#[derive(Default, Serialize)]
pub struct FunctionsOutput {
    pub res: FunctionsResponseData,
}

#[derive(Default, Serialize)]
pub enum HttpStatusCode {
    #[serde(rename = "200")]
    Ok,
    #[serde(rename = "302")]
    Found,
    #[default]
    #[serde(rename = "400")]
    BadRequest,
    #[serde(rename = "401")]
    Unauthorized,
    #[serde(rename = "404")]
    NotFound,
}

#[derive(Default, Serialize)]
pub struct FunctionsResponseData {
    pub body: String,
    #[serde(rename = "statusCode")]
    pub status_code: HttpStatusCode,
    pub headers: HashMap<String, String>,
}
