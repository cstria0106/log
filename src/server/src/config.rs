use std::io::Read;

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
    pub fn from_str(toml: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(toml)
    }

    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut config_file = match std::fs::File::open(path) {
            Ok(file) => file,
            Err(e) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("could not open config file: {}", e),
                )));
            }
        };

        let metadata = config_file.metadata()?;
        if !metadata.is_file() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "it's not a valid file",
            )));
        }

        let mut toml = String::new();
        config_file.read_to_string(&mut toml)?;

        Self::from_str(&toml).map_err(|e| e.into())
    }
}
