use std::io::prelude::*;

use chrono::{Datelike, Timelike, Utc};
use flate2::write::GzEncoder;
use flate2::Compression;
use s3::Bucket;

use crate::{log::Log, logger::Logger};

pub struct S3Logger {
    memory_logs: Vec<Log>,
    bucket: Bucket,
    stdout: bool,
}

impl S3Logger {
    pub fn new(bucket: Bucket, stdout: Option<bool>) -> S3Logger {
        S3Logger {
            memory_logs: Vec::with_capacity(40960),
            bucket,
            stdout: match stdout {
                Some(e) => e,
                _ => false,
            },
        }
    }

    pub fn upload(&self) {
        let last_log = self.memory_logs.last();

        match last_log {
            Some(last_log) => {
                let mut encoder = GzEncoder::new(vec![], Compression::best());

                for log in self.memory_logs.iter() {
                    encoder
                        .write_fmt(format_args!("{}\n", log.message()))
                        .unwrap();
                }

                let filename = last_log.timestamp().format("%Y%m%d.log.gz").to_string();
                self.bucket
                    .put_object_blocking(filename, &encoder.finish().unwrap())
                    .unwrap();
            }
            _ => {}
        }
    }

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
                    && time.day() <= now.day()
                    && time.hour() <= now.hour()
                    && time.minute() < now.minute()
                {
                    if self.stdout {
                        println!("New log started");
                        println!("Last log time: {}", time);
                    }

                    self.upload();
                    self.memory_logs.clear();
                }
            }
            _ => {}
        }

        if self.stdout {
            println!("{} ({})", log.message(), log.timestamp());
        }

        self.memory_logs.push(log);
    }
}
