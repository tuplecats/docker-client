use crate::volume::VolumeInfo;
use serde::{Deserialize, Deserializer};

///TODO doc
#[derive(Debug, Deserialize)]
pub struct VolumesList {

    #[serde(rename = "Volumes", deserialize_with = "nullable_priority_seq_info")]
    volumes: Vec<VolumeInfo>,

    #[serde(rename = "Warnings", deserialize_with = "nullable_priority_seq_str")]
    warnings: Vec<String>,
}

fn nullable_priority_seq_info<'de, D>(deserializer: D) -> Result<Vec<VolumeInfo>, D::Error>
    where D: Deserializer<'de>
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or(Vec::new()))
}

fn nullable_priority_seq_str<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where D: Deserializer<'de>
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or(Vec::new()))
}