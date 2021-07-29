use gitlab::Gitlab;
use serde::de::Deserializer;
use serde::Deserialize;
use serde_json::Value;

use super::review_lifetime_report::ReviewLifetimeReportParams;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(tag = "type", rename_all = "kebab-case")]
#[serde(remote = "ReportParams")]
pub enum ReportParams {
    ReviewLifetime(ReviewLifetimeReportParams),
    #[serde(skip_deserializing)]
    Unknown,
}

impl<'de> Deserialize<'de> for ReportParams {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        let deserialized = Self::deserialize(&value).unwrap_or_else(|_| {
            //tracing::warn!(message = "unknown-variant", variant = ?s);
            Self::Unknown
        });
        Ok(deserialized)
    }
}

pub fn make(maker: ReportParams, client: &Gitlab) {
    match maker {
        ReportParams::ReviewLifetime(params) => params.make_report(client),
        _ => println!("Unknown report params"),
    }
}

pub trait ReportMaker {
    fn make_report(&self, client: &Gitlab);
}
