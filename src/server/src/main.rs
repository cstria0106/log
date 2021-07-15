use chrono::Utc;
use console_device::ConsoleDevice;
use log::{
    grpc::{logger_service_server::LoggerServiceServer, ping_service_server::PingServiceServer},
    log::{Level, Log},
};
use logger::Logger;
use logger_rpc::MyLoggerService;
use ping_rpc::MyPingService;
use s3::{creds::Credentials, Bucket};
use s3_device::S3Device;
use std::{fs::File, io::BufReader};

#[path = "device/console_device.rs"]
mod console_device;
mod device;
mod logger;
#[path = "rpc/logger_rpc.rs"]
mod logger_rpc;
#[path = "rpc/ping_rpc.rs"]
mod ping_rpc;
#[path = "device/s3_device.rs"]
mod s3_device;

/// Configuration struct.
#[derive(serde::Serialize, serde::Deserialize)]
struct Config {
    id: String,
    key: String,
    bucket: String,
}

#[tokio::main]
async fn main() {
    println!("wow");

    // Load configuration from "config.json".
    let config: Config = serde_json::from_reader(BufReader::new(
        File::open("config.json").expect("can't open configuration file"),
    ))
    .expect("can't read configuration file");

    // Define S3 bucket.
    let bucket = Bucket::new(
        &config.bucket,
        "ap-northeast-2".parse().unwrap(),
        Credentials::new(Some(&config.id), Some(&config.key), None, None, None).unwrap(),
    )
    .unwrap();

    // Test S3 bucket.
    bucket.list_blocking(String::new(), None).unwrap();

    // Create logger.
    let mut logger = Logger::new()
        .add_device(Box::new(S3Device::new(bucket)))
        .add_device(Box::new(ConsoleDevice::new()));

    // Log for test.
    logger.log(Log::new(
        Level::Info,
        &"Now starting logging server.".to_string(),
        None,
        Utc::now(),
    ));

    // Start tonic server and wait forever.
    tonic::transport::Server::builder()
        .add_service(LoggerServiceServer::new(MyLoggerService::new(logger)))
        .add_service(PingServiceServer::new(MyPingService {}))
        .serve("[::1]:50051".parse().unwrap())
        .await
        .unwrap();
}
