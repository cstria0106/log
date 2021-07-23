use chrono::{Date, DateTime, Local, Utc};

use futures::{pin_mut, stream};
use log::log::{Level, Log};

use crate::device::{Device, DeviceError};

type Follower = tokio::sync::mpsc::Sender<Log>;

/// Logger holds log data for a day
/// and write log into log devices.
pub struct Logger {
    today_logs: Vec<Log>,
    devices: Vec<Box<dyn Device + Send + Sync>>,
    followers: Vec<(u64, Follower)>,
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            today_logs: Vec::new(),
            devices: Vec::new(),
            followers: Vec::new(),
        }
    }

    /// Add device and return itself.
    pub fn add_device(mut self, device: Box<dyn Device + Send + Sync>) -> Self {
        self.devices.push(device);
        self
    }

    /// Log in every devices and return occurred errors.
    pub async fn log(&mut self, log: Log) -> Vec<DeviceError> {
        for device in self.devices.iter_mut() {
            device.log(&log).await;
        }

        let mut disconnected: Vec<u64> = Vec::new();
        for (id, follower) in self.followers.iter_mut() {
            let follower = follower.clone();
            let log = log.clone();
            match follower.send(log).await {
                Err(_) => {
                    disconnected.push(*id);
                }
                _ => {}
            };
        }

        self.followers.retain(|(id, _)| !disconnected.contains(id));

        /// Check that number of days in duration between a and b is more than one day.
        fn is_after_a_day(a: &DateTime<Utc>, b: &DateTime<Utc>) -> bool {
            (a.with_timezone(&Local).date() - b.with_timezone(&Local).date())
                .num_days()
                .abs()
                > 0
        }

        // Get last log.
        let last_log = self.today_logs.last();

        // If this is not first log, check timestamp between last log and current log.
        let errors = if let Some(last_log) = last_log {
            let time = &last_log.timestamp;
            let now = &log.timestamp;

            // If last log is old, then store and clear logs stored in memory.
            let errors = if is_after_a_day(time, now) {
                let mut errors = Vec::with_capacity(self.devices.len());

                for device in self.devices.iter_mut() {
                    // Store and collect device errors.
                    if let Err(e) = device.store(&self.today_logs).await {
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

    pub async fn get(&self, date: &Date<Utc>, levels: Option<&[Level]>) -> Option<Vec<Log>> {
        if date == &Utc::now().date() {
            return Some(self.today_logs.clone());
        }

        use stream::StreamExt;
        let s = stream::iter(&self.devices)
            .filter_map(|device| async move { device.get(date, levels).await.unwrap() });

        pin_mut!(s);
        s.next().await
    }

    pub fn follow(&mut self, follower: Follower) {
        self.followers
            .push((self.followers.last().map_or(0, |(id, _)| id + 1), follower));
    }
}
