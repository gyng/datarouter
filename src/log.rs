use chrono::prelude::*;

#[derive(Debug, Clone)]
pub struct Log {
    pub timestamp: DateTime<UTC>, // rust-postgres does not support chrono 0.4 yet
    pub payload: String,
    pub label: Option<String>,
}

impl Log {
    pub fn new(payload: String, label: Option<String>) -> Log {
        Log {
            timestamp: UTC::now(),
            payload: payload,
            label: label,
        }
    }
}
