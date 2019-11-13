use std::collections::HashMap;
use serde::{Deserialize, Deserializer};

/// TODO doc
#[derive(Deserialize, Debug)]
pub struct ShortImageInfo {

    #[serde(rename = "Id")]
    id: String,

    #[serde(rename = "ParentId")]
    parent_id: String,

    #[serde(rename = "RepoTags")]
    repo_tags: Vec<String>,

    #[serde(rename = "RepoDigests")]
    repo_digests: Vec<String>,

    #[serde(rename = "Created")]
    created: i64,

    #[serde(rename = "Size")]
    size: i64,

    #[serde(rename = "SharedSize")]
    shared_size: i64,

    #[serde(rename = "VirtualSize")]
    virtual_size: i64,

    #[serde(rename = "Labels", deserialize_with = "nullable_priority_hash")]
    labels: HashMap<String, String>,

    #[serde(rename = "Containers")]
    containers: i64,

}

fn nullable_priority_hash<'de, D>(deserializer: D) -> Result<HashMap<String, String>, D::Error>
    where D: Deserializer<'de>
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or(Default::default()))
}