use chrono::NaiveDate;
use clap::ArgMatches;
use log::{
    grpc::{logger_service_client::LoggerServiceClient, GetRequest},
    log::Log,
};

use crate::config::Config;

pub async fn list(
    args: &ArgMatches<'_>,
    config: &Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = LoggerServiceClient::connect(format!(
        "http://{}:{}",
        config.ip,
        config.port.unwrap_or(50051)
    ))
    .await?;

    let date_string = args
        .value_of("date")
        .map(|date| date.to_string())
        .unwrap_or(chrono::Utc::now().format("%F").to_string());

    NaiveDate::parse_from_str(&date_string, "%F")?;
    let logs = client
        .get(GetRequest { date: date_string })
        .await?
        .into_inner()
        .logs;

    let highlighter = toml_highlighter::Highlighter::new();
    let mut buffer = String::new();
    for log in logs.iter().map(|log| Log::from_grpc_log(log)) {
        match log {
            Ok(log) => buffer.push_str(&log.to_pretty_string(&highlighter)),
            Err(e) => buffer.push_str(&format!("invalid log: {}", e)),
        }
        buffer.push('\n');
    }

    println!("{}", buffer);

    Ok(())
}
