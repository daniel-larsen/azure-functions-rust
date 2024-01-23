use crate::InputBinding;
use serde::Deserialize;
use std::collections::HashMap;
use time::OffsetDateTime;

#[derive(Deserialize)]
pub struct TimerPayload {
    #[serde(rename = "Data")]
    pub data: TimerPayloadData,
    #[serde(rename = "Metadata")]
    pub metadata: TimerPayloadMetadata,
}

impl TimerPayload {
    pub fn method_name(&self) -> &str {
        self.metadata.sys.method_name.as_str()
    }
}

#[derive(Deserialize)]
pub struct TimerPayloadData {
    pub timer: TimerPayloadDataTimer,
    #[serde(flatten)]
    pub inputs: HashMap<String, InputBinding>,
}

#[derive(Deserialize)]
pub struct TimerPayloadDataTimer {
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
    #[serde(rename = "Last", with = "time::serde::rfc3339::option")]
    pub last: Option<OffsetDateTime>,
    #[serde(rename = "Next", with = "time::serde::rfc3339")]
    pub next: OffsetDateTime,
    #[serde(rename = "LastUpdated", with = "time::serde::rfc3339::option")]
    pub last_updated: Option<OffsetDateTime>,
}

#[derive(Deserialize)]
pub struct TimerPayloadMetadata {
    pub sys: TimerPayloadMetadataSys,
}

#[derive(Deserialize)]
pub struct TimerPayloadMetadataSys {
    #[serde(rename = "MethodName")]
    pub method_name: String,
    #[serde(rename = "UtcNow", with = "time::serde::rfc3339")]
    pub utc_now: OffsetDateTime,
    #[serde(rename = "RandGuid")]
    pub rand_guid: uuid::Uuid,
}
