use crate::reports::report::ReportType;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Gitlab {
    pub host: String,
    pub apikey: String,
    // pub project_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Report {
    pub report_type: ReportType,
    //pub project_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub gitlab: Gitlab,
    pub reports: Vec<Report>,
}

impl Config {
    pub fn new<S: Into<String>>(config: S) -> Result<Self, config::ConfigError> {
        let mut settings = config::Config::new();
        settings.merge(config::File::with_name(&config.into()))?;
        settings.try_into()
    }
}
