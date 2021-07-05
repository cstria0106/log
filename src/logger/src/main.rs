pub mod console_logger;
pub mod log;
pub mod logger;
pub mod rpc;
pub mod s3_logger;
pub mod grpc {
    tonic::include_proto!("logger");
}

use rpc::MyLoggerService;
use s3_logger::S3Logger;

use s3::{creds::Credentials, Bucket};
use std::{fs::File, io::BufReader};

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

    // Use S3 logger.
    let logger = S3Logger::new(bucket, Some(true));

    // Start tonic server and wait forever.
    tonic::transport::Server::builder()
        .add_service(grpc::logger_service_server::LoggerServiceServer::new(
            MyLoggerService::new(logger),
        ))
        .serve("[::1]:50051".parse().unwrap())
        .await
        .unwrap();
}
