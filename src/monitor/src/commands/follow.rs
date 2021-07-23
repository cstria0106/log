use clap::ArgMatches;
use log::{
    grpc::{logger_service_client::LoggerServiceClient, FollowRequest},
    log::Log,
};

use crate::config::Config;

pub async fn follow(_: &ArgMatches<'_>, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = LoggerServiceClient::connect(format!(
        "http://{}:{}",
        config.ip,
        config.port.unwrap_or(50051)
    ))
    .await?;

    let mut stream = client.follow(FollowRequest {}).await?.into_inner();
    let highlighter = toml_highlighter::Highlighter::new();

    while let Some(message) = stream.message().await? {
        if let Some(log) = message.log {
            let log = Log::from_grpc_log(&log)?;
            println!("{}", log.to_pretty_string(&highlighter));
        }
    }

    Ok(())
}
