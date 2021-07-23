use chrono::Utc;
use console_device::ConsoleDevice;
use log::{
    grpc::{logger_service_server::LoggerServiceServer, ping_service_server::PingServiceServer},
    log::{Level, Log},
};
use logger::Logger;
use logger_rpc::MyLoggerService;
use ping_rpc::MyPingService;
use s3_device::S3Device;

use crate::config::Config;

mod config;
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

#[tokio::main]
async fn main() {
    let config = Config::from_file("config.json").expect("could not read config");

    // Create logger.
    let mut logger = Logger::new()
        .add_device(Box::new(
            S3Device::new(&config)
                .await
                .expect("could not create S3 device"),
        ))
        .add_device(Box::new(ConsoleDevice::new()));

    // Log for test.
    let errors = logger
        .log(Log::new(
            Level::Info,
            &"Now starting logging server.".to_string(),
            None,
            Utc::now(),
        ))
        .await;

    if !errors.is_empty() {
        eprintln!("error ocurred while logging for test");
        for (index, error) in errors.iter().enumerate() {
            eprintln!("{}: {}", index, error.to_string());
        }
        std::process::exit(1);
    }

    // Start tonic server and wait forever.
    tonic::transport::Server::builder()
        .add_service(LoggerServiceServer::new(MyLoggerService::new(logger)))
        .add_service(PingServiceServer::new(MyPingService {}))
        .serve(
            format!(
                "{}:{}",
                config.host.unwrap_or("[::1]".to_string()),
                config.port.unwrap_or(50051)
            )
            .parse()
            .unwrap(),
        )
        .await
        .unwrap();
}
