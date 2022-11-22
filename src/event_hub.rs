use serde::Deserialize;
use time::OffsetDateTime;

#[derive(Deserialize)]
pub struct EventHubPayloadSystemProperties {
    #[serde(rename = "iothub-connection-device-id")]
    pub device_id: String,
}

#[derive(Deserialize)]
pub struct EventHubPayloadMetadata {
    #[serde(rename = "EnqueuedTimeUtc", with = "time::serde::rfc3339")]
    pub enqueued_time_utc: OffsetDateTime,
    #[serde(rename = "SystemProperties")]
    pub properties: EventHubPayloadSystemProperties,
}

#[derive(Deserialize)]
pub struct EventHubPayload {
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "Metadata")]
    pub metadata: EventHubPayloadMetadata,
}

// impl EventHubPayload {
//     pub fn method_name(&self) -> &str {
//         return self.metadata.sys.method_name.as_str();
//     }
// }
