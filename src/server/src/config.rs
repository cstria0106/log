/// Configuration struct.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub id: String,
    pub key: String,
    pub bucket: String,
    pub host: Option<String>,
    pub port: Option<u16>,
}

impl Config {
    pub fn from_str(toml: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(toml)
    }
}
