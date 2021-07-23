use std::env;
use std::io::prelude::*;

use async_trait::async_trait;
use chrono::{Date, Utc};
use flate2::Compression;
use flate2::{bufread::GzDecoder, write::GzEncoder};
use log::log::{Level, Log};
use rusoto_core::credential::EnvironmentProvider;
use rusoto_core::{HttpClient, RusotoError};
use rusoto_s3::{Bucket, GetObjectError, GetObjectRequest, PutObjectRequest, S3Client, S3};
use tokio::io::AsyncReadExt;

use crate::config::Config;
use crate::device::{self, Device, DeviceError};

pub struct S3Device {
    client: S3Client,
    bucket: Bucket,
}

impl S3Device {
    pub async fn new(config: &Config) -> Result<S3Device, Box<dyn std::error::Error>> {
        // Get S3 bucket.
        env::set_var("AWS_ACCESS_KEY_ID", config.id.clone());
        env::set_var("AWS_SECRET_ACCESS_KEY", config.key.clone());
        let client = S3Client::new_with(
            HttpClient::new().unwrap(),
            EnvironmentProvider::default(),
            rusoto_core::Region::ApNortheast2,
        );

        let bucket = client
            .list_buckets()
            .await?
            .buckets
            .unwrap()
            .into_iter()
            .find(|bucket| bucket.name == Some(config.bucket.clone()))
            .expect(&format!("no bucket named '{}'", config.bucket));

        Ok(S3Device { client, bucket })
    }
}

#[async_trait]
impl Device for S3Device {
    /// Do nothing.
    async fn log(&mut self, _: &Log) {}

    /// Store log into S3.
    async fn store(&mut self, logs: &Vec<Log>) -> device::Result<Option<String>> {
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
            let filename = last_log.timestamp.format("%F.log.gz").to_string();

            // Upload.
            self.client
                .put_object(PutObjectRequest {
                    bucket: self.bucket.name.clone().unwrap(),
                    key: filename.clone(),
                    body: Some(encoded.into()),
                    ..Default::default()
                })
                .await
                .map_err(|e| {
                    DeviceError::new(format!("error occurred while uploading S3: {}", e))
                })?;

            return Ok(Some(filename));
        }

        Ok(None)
    }

    /// Get logs of certain date from S3.
    async fn get(
        &self,
        date: &Date<Utc>,
        levels: Option<&[Level]>,
    ) -> device::Result<Option<Vec<Log>>> {
        // List and get zipped logs.
        let result = self
            .client
            .get_object(GetObjectRequest {
                bucket: self.bucket.name.clone().unwrap(),
                key: date.format("%F.log.gz").to_string(),
                ..Default::default()
            })
            .await;

        if let Err(e) = &result {
            if let RusotoError::Service(e) = e {
                if let GetObjectError::NoSuchKey(_) = e {
                    return Ok(None);
                }
            }
        }

        let result = result.map_err(|e| DeviceError::new(format!("could not fetch log: {}", e)))?;

        let mut body_read = result.body.unwrap().into_async_read();
        let mut body = Vec::with_capacity(result.content_length.unwrap_or(0) as usize);
        body_read
            .read_to_end(&mut body)
            .await
            .map_err(|e| DeviceError::new(format!("could not read log body: {}", e)))?;

        // Unzip and parse.
        let mut decoder = GzDecoder::new(&body[..]);
        let mut decoded = Vec::new();
        decoder
            .read_to_end(&mut decoded)
            .map_err(|e| DeviceError::new(format!("error occurred while decompressing: {}", e)))?;

        let mut logs: Vec<Log> = bincode::deserialize(&decoded)
            .map_err(|e| DeviceError::new(format!("error occurred while deserializing: {}", e)))?;

        // Filter level.
        if let Some(levels) = levels {
            logs.retain(|log| levels.contains(&log.level))
        }

        Ok(Some(logs))
    }
}
