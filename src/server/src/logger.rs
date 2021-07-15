use chrono::{Date, DateTime, Local, Utc};

use log::log::{Level, Log};

use crate::device::{Device, DeviceError};

/// Logger holds log data for a day
/// and write log into log devices.
pub struct Logger {
    today_logs: Vec<Log>,
    devices: Vec<Box<dyn Device + Send>>,
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            today_logs: Vec::new(),
            devices: Vec::new(),
        }
    }

    /// Add device and return itself.
    pub fn add_device(mut self, device: Box<dyn Device + Send>) -> Self {
        self.devices.push(device);
        self
    }

    /// Log in every devices and return occurred errors.
    pub fn log(&mut self, log: Log) -> Vec<DeviceError> {
        for device in self.devices.iter_mut() {
            device.log(&log);
        }

        /// Check that number of days in duration between a and b is more than one day.
        fn is_after_a_day(a: &DateTime<Utc>, b: &DateTime<Utc>) -> bool {
            (a.with_timezone(&Local).date() - b.with_timezone(&Local).date())
                .num_days()
                .abs()
                > 0
        }

        // Temporary check function for development.
        // fn is_after_a_day(_: &DateTime<Utc>, _: &DateTime<Utc>) -> bool {
        //     true
        // }

        // Get last log.
        let last_log = self.today_logs.last();

        // If this is not first log, check timestamp between last log and current log.
        let errors = if let Some(last_log) = last_log {
            let time = last_log.timestamp();
            let now = log.timestamp();

            // If last log is old, then store and clear logs stored in memory.
            let errors = if is_after_a_day(time, now) {
                let mut errors = Vec::with_capacity(self.devices.len());

                for device in self.devices.iter_mut() {
                    // Store and collect device errors.
                    if let Err(e) = device.store(&self.today_logs) {
                        errors.push(e);
                    }
                }

                self.today_logs.clear();
                errors
            } else {
                Vec::new()
            };

            errors
        } else {
            Vec::new()
        };

        // Push log into memory.
        self.today_logs.push(log);

        errors
    }

    pub fn get(&self, date: &Date<Utc>, levels: Option<&[Level]>) -> Option<Vec<Log>> {
        if date == &Utc::now().date() {
            return Some(self.today_logs.clone());
        }

        self.devices
            .iter()
            .filter_map(|device| device.get(date, levels).ok())
            .find_map(|device| device)
    }
}
