use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TimesheetRecord {
    pub submitted: String,
    pub project: String,
    pub start: String, // chrono::DateTime<chrono::Local>
    pub end: String, //chrono::DateTime<chrono::Local>
    pub notes: String,
}
