pub mod console_logger;
pub mod log;
pub mod logger;
pub mod logger_rpc;
pub mod ping_rpc;
pub mod s3_logger;

pub mod grpc {
    tonic::include_proto!("logger");
    tonic::include_proto!("ping");
}

use logger_rpc::MyLoggerService;
use ping_rpc::MyPingService;
use s3_logger::S3Logger;

use s3::{creds::Credentials, Bucket};
use std::{fs::File, io::BufReader};

use crate::{
    console_logger::ConsoleLogger,
    log::{Log, LogLevel},
    logger::Logger,
};

/// Configuration struct.
#[derive(serde::Serialize, serde::Deserialize)]
struct Config {
    id: String,
    key: String,
    bucket: String,
}

#[tokio::main]
async fn main() {
    // Load configuration from "config.json".
    let config: Config =
        serde_json::from_reader(BufReader::new(File::open("config.json").unwrap())).unwrap();

    // Define S3 bucket.
    let bucket = Bucket::new(
        &config.bucket,
        "ap-northeast-2".parse().unwrap(),
        Credentials::new(Some(&config.id), Some(&config.key), None, None, None).unwrap(),
    )
    .unwrap();

    // Create loggers.
    let s3_logger = S3Logger::new(bucket);
    s3_logger.list().unwrap();

    let console_logger = ConsoleLogger::new();

    let mut loggers: Vec<Box<dyn Logger + Send>> =
        vec![Box::new(s3_logger), Box::new(console_logger)];

    // Log for test.
    for logger in loggers.iter_mut() {
        logger.log(Log::new(
            LogLevel::Info,
            &"Now starting logging server.".to_string(),
            None,
        ));
    }

    // Start tonic server and wait forever.
    tonic::transport::Server::builder()
        .add_service(grpc::logger_service_server::LoggerServiceServer::new(
            MyLoggerService::new(loggers),
        ))
        .add_service(grpc::ping_service_server::PingServiceServer::new(
            MyPingService {},
        ))
        .serve("[::1]:50051".parse().unwrap())
        .await
        .unwrap();
}
