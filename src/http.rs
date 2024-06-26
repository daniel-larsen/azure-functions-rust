use crate::InputBinding;
use hyper::{HeaderMap, header::HeaderName};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use time::OffsetDateTime;
use url::Url;
use std::str::FromStr;

#[macro_export]
macro_rules! require_auth {
    ($request_data:expr) => {
        if $request_data.user_id().is_none() {
            let mut response = azure_functions::FunctionsResponse::default();
            response.outputs.res.status_code = azure_functions::HttpStatusCode::Unauthorized;
            return Ok(response);
        }
    };
}

#[macro_export]
macro_rules! require_auth_redirect {
    ($request_data:expr) => {
        if $request_data.user_id().is_none() {
            let mut response = azure_functions::FunctionsResponse::default();
            let login_url =
                "/.auth/login/aad?post_login_redirect_url=".to_owned() + $request_data.url.as_str();
            response.outputs.res.status_code = azure_functions::HttpStatusCode::Found;
            response.outputs.res.body = login_url.to_string();
            response
                .outputs
                .res
                .headers
                .insert("Location".to_string(), login_url.to_string());
            return Ok(response);
        }
    };
}

#[derive(Deserialize)]
pub struct HttpPayload {
    #[serde(rename = "Data")]
    pub data: HttpPayloadData,
    #[serde(rename = "Metadata")]
    pub metadata: HttpPayloadMetadata,
}

impl HttpPayload {
    pub fn method_name(&self) -> &str {
        self.metadata.sys.method_name.as_str()
    }
}

#[derive(Deserialize)]
pub struct HttpPayloadData {
    pub req: DataRequest,
    #[serde(flatten)]
    pub inputs: HashMap<String, InputBinding>,
}

#[derive(Deserialize, Clone, Copy, PartialEq)]
pub enum HttpMethod {
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "DELETE")]
    Delete,
    #[serde(rename = "HEAD")]
    Head,
    #[serde(rename = "PATCH")]
    Patch,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "OPTIONS")]
    Options,
    #[serde(rename = "TRACE")]
    Trace,
}

#[derive(Deserialize)]
pub struct DataRequest {
    #[serde(rename = "Url")]
    pub url: Url,
    #[serde(rename = "Method")]
    pub method: HttpMethod,
    #[serde(rename = "Query")]
    pub query: HashMap<String, String>,
    #[serde(rename = "Headers", deserialize_with = "deserialize_header_map")]
    pub headers: HeaderMap<Vec<String>>,
    #[serde(rename = "Body")]
    pub body: Option<String>,
}

impl DataRequest {
    pub fn user_id(&self) -> Option<String> {
        let header_user_id = self.headers.get("x-ms-client-principal-id");
        header_user_id.map(|header_user_id| header_user_id[0].clone())
    }

    pub fn user_name(&self) -> Option<String> {
        let header_username = self.headers.get("x-ms-client-principal-name");
        header_username.map(|header_username| header_username[0].clone())
    }
}

#[derive(Deserialize)]
pub struct HttpPayloadMetadata {
    pub sys: HttpPayloadMetadataSys,
}

#[derive(Deserialize)]
pub struct HttpPayloadMetadataSys {
    #[serde(rename = "MethodName")]
    pub method_name: String,
    #[serde(rename = "UtcNow", with = "time::serde::rfc3339")]
    pub utc_now: OffsetDateTime,
    #[serde(rename = "RandGuid")]
    pub rand_guid: uuid::Uuid,
}

fn deserialize_header_map<'de, D>(deserializer: D) -> Result<HeaderMap<Vec<String>>, D::Error>
where D: Deserializer<'de> {
    let map: HashMap<String, Vec<String>> = HashMap::deserialize(deserializer)?;
    let mut header_map: HeaderMap<Vec<String>> = HeaderMap::<Vec<String>>::default();

    for header in map.into_iter() {
        header_map.append(HeaderName::from_str(&header.0).map_err(serde::de::Error::custom)?, header.1);
    };
    
    Ok(header_map)
}