use anyhow::{bail, Context, Result};
use chrono::Utc;
use console_device::ConsoleDevice;
use log::{
    log::{Level, Log},
    proto::{logger_service_server::LoggerServiceServer, ping_service_server::PingServiceServer},
};
use logger::Logger;
use logger_rpc::MyLoggerService;
use ping_rpc::MyPingService;
use s3_device::S3Device;

use crate::{cli::get_arguments, config::Config, device::Device};

mod cli;
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
async fn main() -> Result<()> {
    let args = get_arguments();

    // Load config.
    let config_file_path = args.value_of("config").unwrap();
    let config = Config::from_file(config_file_path).context("Failed to load config file")?;

    // Create devices.
    let mut devices: Vec<Box<dyn Device + Send + Sync>> = Vec::new();

    for device_name in config
        .devices
        .as_ref()
        .unwrap_or(&vec!["console".to_string()])
        .iter()
        .map(|s| s.trim())
    {
        devices.push(match &device_name[..] {
            "console" => Box::new(ConsoleDevice::new()),
            "s3" => Box::new(
                S3Device::new(&config)
                    .await
                    .context("Could not create S3 device")?,
            ),
            _ => bail!("Unknown device: {}", device_name),
        });
    }

    // Create logger.
    let mut logger = devices
        .into_iter()
        .fold(Logger::new(), |logger, device| logger.add_device(device));

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
        eprintln!("Error ocurred while logging for test");
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
                config.host.unwrap_or("127.0.0.1".to_string()),
                config.port.unwrap_or(50051)
            )
            .parse()?,
        )
        .await?;

    Ok(())
}
