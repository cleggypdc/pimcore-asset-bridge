use serde::Serialize;
use chrono::{DateTime, Utc};

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FileEventType {
    Created,
    Modified,
    Deleted,
    Renamed,
    MovedOut,
    MovedIn,
}

#[derive(Serialize)]
pub struct FileEvent {
    pub event: FileEventType,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    pub timestamp: DateTime<Utc>,
}
