use serde::Deserialize;
use std::collections::HashMap;
use time::OffsetDateTime;
use url::Url;

#[macro_export]
macro_rules! require_auth {
    ($request_data:expr,$response:expr) => {
        if $request_data.user_id().is_none() {
            $response.outputs.res.status_code = HttpStatusCode::Unauthorized;
            $response.outputs.res.body = "".to_string();
            return;
        }
    };
}

#[macro_export]
macro_rules! require_auth_redirect {
    ($request_data:expr,$response:expr) => {
        if $request_data.user_id().is_none() {
            let login_url =
                "/.auth/login/aad?post_login_redirect_url=".to_owned() + $request_data.url.as_str();
            $response.outputs.res.status_code = HttpStatusCode::Found;
            $response.outputs.res.body = login_url.to_string();
            $response
                .outputs
                .res
                .headers
                .insert("Location".to_string(), login_url.to_string());
            return;
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

#[derive(Deserialize)]
pub struct HttpPayloadData {
    pub req: DataRequest,
}

#[derive(Deserialize, Clone, Copy)]
pub enum HttpMethod {
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "POST")]
    Post,
}

#[derive(Deserialize)]
pub struct DataRequest {
    #[serde(rename = "Url")]
    pub url: Url,
    #[serde(rename = "Method")]
    pub method: HttpMethod,
    #[serde(rename = "Query")]
    pub query: HashMap<String, String>,
    #[serde(rename = "Headers")]
    pub headers: HashMap<String, Vec<String>>,
}

impl DataRequest {
    pub fn user_id(&self) -> Option<String> {
        let header_user_id = self.headers.get("X-MS-CLIENT-PRINCIPAL-ID");
        match header_user_id {
            Some(header_user_id) => return Some(header_user_id[0].clone()),
            None => return None, // Return None if user id not found
        };
    }

    pub fn user_name(&self) -> Option<String> {
        let header_username = self.headers.get("X-MS-CLIENT-PRINCIPAL-NAME");
        match header_username {
            Some(header_username) => return Some(header_username[0].clone()),
            None => return None, // Return None if username not found
        };
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
    pub utc_now: OffsetDateTime, // "UtcNow": "2022-10-26T03:32:55.7362251Z",
    #[serde(rename = "RandGuid")]
    pub rand_guid: String,
}
