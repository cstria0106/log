use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Log {
    message: String,
    timestamp: DateTime<Utc>,
}

impl<'a> Log {
    pub fn from_string(message: &String) -> Log {
        return Log {
            message: message.clone(),
            timestamp: Utc::now(),
        };
    }

    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }
}
