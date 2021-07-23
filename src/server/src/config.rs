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
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Load configuration from "config.json".
        serde_json::from_reader(std::io::BufReader::new(std::fs::File::open(path)?))
            .map_err(|e| e.to_string().into())
    }
}
