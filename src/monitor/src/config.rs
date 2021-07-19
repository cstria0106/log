use std::{fs, io::Read, process};

use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    ip: String,
    port: Option<u16>,
}

impl Config {
    fn from_str(toml: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(toml)
    }

    fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_file = match fs::File::open(path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Could not open config file '{}': {}", path, e);
                process::exit(1);
            }
        };

        let metadata = config_file.metadata()?;
        if !metadata.is_file() {
            eprintln!("'{}' is not a valid file", path);
            process::exit(1);
        }

        let mut toml = String::new();
        config_file.read_to_string(&mut toml)?;

        Self::from_str(&toml)
    }
}
