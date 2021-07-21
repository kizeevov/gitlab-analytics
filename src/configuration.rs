use crate::reports::report::ReportParams;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Gitlab {
    pub host: String,
    pub apikey: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub gitlab: Gitlab,
    pub reports: Vec<ReportParams>,
}

impl Config {
    pub fn new<S: Into<String>>(config: S) -> Result<Self, config::ConfigError> {
        let mut settings = config::Config::new();
        settings.merge(config::File::with_name(&config.into()))?;
        settings.try_into()
    }
}
