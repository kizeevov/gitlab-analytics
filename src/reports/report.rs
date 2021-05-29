use serde::de::{value, Deserializer, IntoDeserializer};
use serde::Deserialize;
use std::str::FromStr;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[serde(remote = "ReportType")]
pub enum ReportType {
    ReviewLifetime,
    #[serde(skip_deserializing)]
    Unknown(String),
}

impl FromStr for ReportType {
    type Err = value::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::deserialize(s.into_deserializer())
    }
}

impl<'de> Deserialize<'de> for ReportType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let deserialized = Self::from_str(&s).unwrap_or_else(|_| {
            //tracing::warn!(message = "unknown-variant", variant = ?s);
            Self::Unknown(s)
        });
        Ok(deserialized)
    }
}
