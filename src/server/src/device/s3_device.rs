use std::io::prelude::*;

use chrono::{Date, Utc};
use flate2::Compression;
use flate2::{bufread::GzDecoder, write::GzEncoder};
use log::log::{Level, Log};
use s3::Bucket;

use crate::device::{self, Device, DeviceError};

pub struct S3Device {
    bucket: Bucket,
}

impl S3Device {
    pub fn new(bucket: Bucket) -> S3Device {
        S3Device { bucket }
    }
}

impl Device for S3Device {
    fn log(&mut self, _: &Log) {}

    fn store(&mut self, logs: &Vec<Log>) -> device::Result<Option<String>> {
        let last_log = logs.last();

        if let Some(last_log) = last_log {
            // Create gzip encoder.
            let mut encoder =
                GzEncoder::new(Vec::with_capacity(logs.len() * 256), Compression::best());

            // Compress and write logs.
            encoder
                .write(&bincode::serialize(&logs).map_err(|e| {
                    DeviceError::new(format!("error occurred while serializing: {}", e))
                })?)
                .map_err(|e| {
                    DeviceError::new(format!("error occurred while compressing: {}", e))
                })?;

            // Complete.
            let encoded = encoder.finish().map_err(|e| {
                DeviceError::new(format!("error occurred while completing encoding: {}", e))
            })?;

            // Format filename.
            let filename = last_log.timestamp().format("%F.log.gz").to_string();

            // Upload.
            let (_, code) = self
                .bucket
                .put_object_blocking(filename.clone(), &encoded)
                .map_err(|e| {
                    DeviceError::new(format!("error occurred while uploading S3: {}", e))
                })?;

            if code != 200 {
                return Err(DeviceError::new(format!(
                    "error occurred while uploading S3: status code is {}",
                    code
                )));
            }

            return Ok(Some(filename));
        }

        Ok(None)
    }

    fn get(&self, date: &Date<Utc>, levels: Option<&[Level]>) -> device::Result<Option<Vec<Log>>> {
        let (buffer, code) = self
            .bucket
            .get_object_blocking(format!("/{}", date.format("%F.log.gz")))
            .map_err(|e| {
                DeviceError::new(format!("error occurred while download from S3: {}", e))
            })?;

        if code == 404 {
            Ok(None)
        } else {
            let mut decoder = GzDecoder::new(&buffer[..]);
            let mut decoded = Vec::new();
            decoder.read_to_end(&mut decoded).map_err(|e| {
                DeviceError::new(format!("error occurred while decompressing: {}", e))
            })?;

            let mut logs: Vec<Log> = bincode::deserialize(&decoded).map_err(|e| {
                DeviceError::new(format!("error occurred while deserializing: {}", e))
            })?;

            if let Some(levels) = levels {
                logs.retain(|log| levels.contains(log.level()))
            }

            Ok(Some(logs))
        }
    }
}
