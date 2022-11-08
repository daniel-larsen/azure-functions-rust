use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct TimerPayload {
    #[serde(rename = "Data")]
    #[serde(flatten)]
    pub data: HashMap<String, TimerPayloadData>,
    #[serde(rename = "Metadata")]
    pub metadata: TimerPayloadMetadata,
}

#[derive(Deserialize)]
pub struct TimerPayloadData {
    #[serde(rename = "Schedule")]
    pub schedule: TimerPayloadDataSchedule,
    #[serde(rename = "ScheduleStatus")]
    pub schedule_status: TimerPayloadDataScheduleStatus,
    #[serde(rename = "IsPastDue")]
    pub is_past_due: bool,
}

#[derive(Deserialize)]
pub struct TimerPayloadDataSchedule {
    #[serde(rename = "AdjustForDST")]
    pub adjust_for_dst: bool,
}

#[derive(Deserialize)]
pub struct TimerPayloadDataScheduleStatus {
    #[serde(rename = "Last")]
    pub last: String,
    #[serde(rename = "Next")]
    pub next: String,
    #[serde(rename = "LastUpdated")]
    pub last_updated: String,
}

#[derive(Deserialize)]
pub struct TimerPayloadMetadata {
    pub sys: TimerPayloadMetadataSys,
}

#[derive(Deserialize)]
pub struct TimerPayloadMetadataSys {
    #[serde(rename = "MethodName")]
    pub method_name: String, // "MethodName": "checkCertificates",
    #[serde(rename = "UtcNow")]
    pub utc_now: String, // "UtcNow": "2022-10-26T03:32:55.7362251Z",
    #[serde(rename = "RandGuid")]
    pub rand_guid: String, // "RandGuid": "7492f2df-883f-4777-b799-bdfc267fc0e7"
}
