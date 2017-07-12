#[derive(Debug, Clone)]
pub struct Log {
    timestamp: u32,
    payload: String,
    label: Option<String>,
}

impl Log {
    pub fn new(payload: String, label: Option<String>) -> Log {
        Log {
            timestamp: 0,
            payload: payload,
            label: label,
        }
    }
}
