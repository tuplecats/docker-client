use std::collections::HashMap;
use serde::{Deserialize, Deserializer};

/// Usage data structure
#[derive(Debug, Deserialize)]
pub struct UsageData {
    #[serde(rename = "Size")]
    size: i64,

    #[serde(rename = "RefCount")]
    ref_count: i64,
}

/// Volume info structure
#[derive(Debug, Deserialize)]
pub struct VolumeInfo {
    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Driver")]
    driver: String,

    #[serde(rename = "Mountpoint")]
    mountpoint: String,

    #[serde(rename = "CreatedAt")]
    created: String,

    #[serde(rename = "Status", deserialize_with = "nullable_priority_hash", default = "Default::default")]
    status: HashMap<String, String>,

    #[serde(rename = "Labels", deserialize_with = "nullable_priority_hash")]
    labels: HashMap<String, String>,

    #[serde(rename = "Scope")]
    scope: String,

    #[serde(rename = "Options", deserialize_with = "nullable_priority_hash")]
    options: HashMap<String, String>,

    #[serde(rename = "UsageData")]
    usage_data: Option<UsageData>,
}

fn nullable_priority_hash<'de, D>(deserializer: D) -> Result<HashMap<String, String>, D::Error>
    where D: Deserializer<'de>
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or(Default::default()))
}