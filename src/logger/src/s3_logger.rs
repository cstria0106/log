use chrono::{DateTime, Datelike, NaiveDateTime, Utc};

use crate::{log::Log, logger::Logger};

pub struct S3Logger {
    memory_logs: Vec<Log>,
}

impl S3Logger {
    pub fn new() -> S3Logger {
        S3Logger {
            memory_logs: Vec::with_capacity(40960),
        }
    }

    pub fn upload() {}

    pub fn download() {}
}

impl Logger for S3Logger {
    fn log(&mut self, log: Log) {
        let last_log = self.memory_logs.last();

        match last_log {
            Some(last_log) => {
                let time = last_log.timestamp();
                let now = Utc::now();

                if time.year() <= now.year()
                    && time.month() <= now.month()
                    && time.day() < now.day()
                {
                    println!("New log started");
                    println!("Last log time: {}", time);

                    self.memory_logs.clear();
                }
            }
            _ => {}
        }

        self.memory_logs.push(log);
        println!("{:?}", self.memory_logs);
    }
}
