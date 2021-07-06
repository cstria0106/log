use std::io::prelude::*;

use chrono::{DateTime, Utc};
use colored::*;
use flate2::write::GzEncoder;
use flate2::Compression;
use s3::Bucket;

use crate::{log::Log, logger::Logger};

pub struct S3Logger {
    memory_logs: Vec<Log>,
    stdout: bool,
    bucket: Bucket,
}

#[derive(Debug)]
pub enum UploadError {
    CompressionError(String),
    S3Error(String),
}

pub type UploadResult = Result<Option<String>, UploadError>;

impl S3Logger {
    pub fn new(bucket: Bucket, stdout: Option<bool>) -> S3Logger {
        // bucket.list_blocking("".to_string(), None).unwrap();

        S3Logger {
            memory_logs: Vec::with_capacity(40960),
            stdout: stdout.unwrap_or(false),
            bucket,
        }
    }

    pub fn upload(&self) -> UploadResult {
        let last_log = self.memory_logs.last();

        if let Some(last_log) = last_log {
            // Create gzip encoder.
            let mut encoder = GzEncoder::new(
                Vec::with_capacity(self.memory_logs.len() * 256),
                Compression::best(),
            );

            // Compress and write logs.
            for log in self.memory_logs.iter() {
                encoder
                    .write_fmt(format_args!("{}\n", log.message()))
                    .map_err(|e| {
                        UploadError::CompressionError(format!(
                            "error occured while compressing: {}",
                            e
                        ))
                    })?;
            }

            // Complete.
            let encoded = encoder.finish().map_err(|e| {
                UploadError::CompressionError(format!(
                    "error occured while completing encoding: {}",
                    e
                ))
            })?;

            // Format filename.
            let filename = last_log.timestamp().format("%F.log.gz").to_string();

            // Upload.
            let (_, code) = self
                .bucket
                .put_object_blocking(filename.clone(), &encoded)
                .map_err(|e| {
                    UploadError::S3Error(format!("error occured while uploading S3: {}", e))
                })?;

            if code != 200 {
                return Err(UploadError::S3Error(format!(
                    "error occured while uploading S3: status code is {}",
                    code
                )));
            }
            return Ok(Some(filename));
        }

        Ok(None)
    }

    pub fn download() {}
}

impl Logger for S3Logger {
    fn log(&mut self, log: Log) {
        // check that number of days in duration between a and b is more than one day.
        // fn is_after_a_day(a: &DateTime<Utc>, b: &DateTime<Utc>) -> bool {
        //     (a.with_timezone(&Local).date() - b.with_timezone(&Local).date())
        //         .num_days()
        //         .abs()
        //         > 0
        // }

        // temporary check function for development.
        fn is_after_a_day(_: &DateTime<Utc>, _: &DateTime<Utc>) -> bool {
            true
        }

        let last_log = self.memory_logs.last();

        // If this is not first log, check timestamp between last log and current log.
        let upload_result = if let Some(last_log) = last_log {
            let time = last_log.timestamp();
            let now = log.timestamp();

            // If last log is old, then upload to S3 and clear logs stored in memory.
            if is_after_a_day(time, now) {
                if self.stdout {
                    println!("{}", "uploading...".bright_black());
                }

                let result = self.upload();
                self.memory_logs.clear();
                result
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        };

        // If use stdout, print log in stdout.
        if self.stdout {
            println!(
                "{} {}",
                log.message(),
                log.timestamp().format("%F %T").to_string().bright_black()
            );
        }

        // Push logs into memory.
        self.memory_logs.push(log);

        // If upload failed, then panic.
        upload_result.unwrap();
    }
}
