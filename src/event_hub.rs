use serde::{Deserialize, Deserializer};
use time::{format_description::well_known::Iso8601, OffsetDateTime, PrimitiveDateTime};

#[derive(Deserialize)]
pub struct EventHubPayloadMetadataSys {
    #[serde(rename = "MethodName")]
    pub method_name: String,
    #[serde(rename = "UtcNow", with = "time::serde::rfc3339")]
    pub utc_now: OffsetDateTime,
    #[serde(rename = "RandGuid")]
    pub rand_guid: uuid::Uuid,
}

#[derive(Deserialize)]
pub struct EventHubPayloadMetadata {
    #[serde(rename = "Offset")]
    pub offset: String,
    #[serde(rename = "SequenceNumber")]
    pub sequence_number: String,
    #[serde(rename = "EnqueuedTimeUtc", deserialize_with = "iso8601_assume_utc")]
    pub enqueued_time_utc: OffsetDateTime,
    pub sys: EventHubPayloadMetadataSys,
}

#[derive(Deserialize)]
pub struct EventHubPayload {
    #[serde(rename = "Data")]
    pub data: String,
    #[serde(rename = "Metadata")]
    pub metadata: EventHubPayloadMetadata,
}

impl EventHubPayload {
    pub fn method_name(&self) -> &str {
        return self.metadata.sys.method_name.as_str();
    }
}

pub fn iso8601_assume_utc<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;
    let primitive =
        PrimitiveDateTime::parse(&buf, &Iso8601::DEFAULT).map_err(serde::de::Error::custom)?;
    Ok(primitive.assume_utc())
}
