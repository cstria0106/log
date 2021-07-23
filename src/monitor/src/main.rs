mod cli;
mod config;
mod monitor;

#[path = "commands/follow.rs"]
mod command_follow;

use std::{error::Error, process};

use monitor::Monitor;

use crate::{cli::get_arguments, config::Config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = get_arguments();

    let config_file_path = args.value_of("config").unwrap();
    let config = match Config::from_file(config_file_path) {
        Err(e) => {
            eprintln!("could not read config file '{}': {}", config_file_path, e);
            process::exit(1);
        }
        Ok(c) => c,
    };

    let monitor = Monitor {};
    if let Err(e) = monitor.run(&args, &config).await {
        println!("error ocurred: {}", e.to_string());
    }

    Ok(())
}
