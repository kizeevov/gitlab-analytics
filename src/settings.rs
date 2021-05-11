use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Gitlab {
    pub host: String,
    pub apikey: String,
    pub project_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub gitlab: Gitlab,
}

impl Settings {
    pub fn new<S: Into<String>>(config: S) -> Result<Self, ConfigError> {
        let mut settings = Config::new();
        settings.merge(File::with_name(&config.into()))?;
        settings.try_into()
    }
}
