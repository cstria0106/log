use async_trait::async_trait;
use chrono::{Date, Utc};
use log::log::{Level, Log};

pub type Result<T> = std::result::Result<T, DeviceError>;

#[derive(Debug)]
pub struct DeviceError(String);

impl DeviceError {
    pub fn new(message: String) -> Self {
        Self(message)
    }
}

impl std::fmt::Display for DeviceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Abstract device for logging.
#[async_trait]
pub trait Device {
    /// Log.
    async fn log(&mut self, log: &Log);

    /// Store memory logs.
    async fn store(&mut self, logs: &Vec<Log>) -> Result<Option<String>>;

    // Get log by UTC date.
    async fn get(&self, date: &Date<Utc>, levels: Option<&[Level]>) -> Result<Option<Vec<Log>>>;
}
