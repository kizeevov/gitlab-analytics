use chrono::{DateTime, Utc};
use serde::de::Deserializer;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(tag = "type", rename_all = "kebab-case")]
#[serde(remote = "ReportParams")]
pub enum ReportParams {
    ReviewLifetime {
        project_name: String,
        datetime_from: DateTime<Utc>,
        datetime_to: DateTime<Utc>,
        assignee: String,
    },
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
