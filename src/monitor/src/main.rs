mod cli;
mod config;
mod monitor;

use std::{error::Error, process};

use crate::{cli::get_arguments, config::Config};

fn main() -> Result<(), Box<dyn Error>> {
    let arguments = get_arguments();

    let config_file_path = arguments.value_of("config").unwrap();
    let config = match Config::from_file(config_file_path) {
        Err(e) => {
            eprintln!("could not read config file '{}': {}", config_file_path, e);
            process::exit(1);
        }
        Ok(c) => c,
    };

    println!("{:?}", config);

    if let Some(s) = arguments.subcommand_matches("list") {
        println!("{:?}", s.value_of("date"));
    }

    Ok(())
}
