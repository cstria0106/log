use std::io::Read;

use anyhow::{ensure, Context, Result};

/// Configuration struct.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub id: String,
    pub key: String,
    pub bucket: String,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub devices: Option<Vec<String>>,
}

impl Config {
    pub fn from_str(toml: &str) -> Result<Self> {
        toml::from_str(toml).map_err(|e| e.into())
    }

    pub fn from_file(path: &str) -> Result<Self> {
        let mut config_file = std::fs::File::open(path).context("failed to open config file")?;

        let metadata = config_file.metadata()?;
        ensure!(metadata.is_file(), "config file is not a file");

        let mut toml = String::new();
        config_file
            .read_to_string(&mut toml)
            .context("failed to read config file")?;

        Self::from_str(&toml)
    }
}
