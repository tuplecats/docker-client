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

impl ShortImageInfo {

    /// TODO doc
    pub fn id(&self) -> String {
        self.id.clone()
    }

    /// TODO doc
    pub fn parent_id(&self) -> String {
        self.parent_id.clone()
    }

    /// TODO doc
    pub fn repo_tags(&self) -> Vec<String> {
        self.repo_tags.clone()
    }

    /// TODO doc
    pub fn repo_digests(&self) -> Vec<String> {
        self.repo_digests.clone()
    }

    /// TODO doc
    pub fn created(&self) -> i64 {
        self.created
    }

    /// TODO doc
    pub fn size(&self) -> i64 {
        self.size
    }

    /// TODO doc
    pub fn shared_size(&self) -> i64 {
        self.shared_size
    }

    /// TODO doc
    pub fn virtual_size(&self) -> i64 {
        self.virtual_size
    }

    /// TODO doc
    pub fn labels(&self) -> HashMap<String, String> {
        self.labels.clone()
    }

    /// TODO doc
    pub fn containers(&self) -> i64 {
        self.containers
    }
}

fn nullable_priority_hash<'de, D>(deserializer: D) -> Result<HashMap<String, String>, D::Error>
    where D: Deserializer<'de>
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or(Default::default()))
}