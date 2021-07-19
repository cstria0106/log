mod config;

use std::{error::Error, fs::File, process};

use clap::{App, AppSettings, Arg, SubCommand};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Log Monitor")
        .settings(&[AppSettings::ColoredHelp, AppSettings::ArgRequiredElseHelp])
        .version("0.0.1")
        .about("An easy log monitor tool")
        .arg(
            Arg::with_name("config")
                .long("config")
                .short("c")
                .value_name("PATH")
                .help("Specifies config file path")
                .default_value("config.toml")
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("list")
                .setting(AppSettings::ColoredHelp)
                .visible_alias("ls")
                .about("List logged dates or logs of certain date"),
        )
        .get_matches();

    let config_file_path = matches.value_of("config").unwrap();
    let config_file = match File::open(config_file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Could not open config file '{}': {}", config_file_path, e);
            process::exit(1);
        }
    };

    let metadata = config_file.metadata()?;
    if !metadata.is_file() {
        eprintln!("'{}' is not a valid file", config_file_path);
        process::exit(1);
    }

    println!("{:?}", config_file);

    Ok(())
}
